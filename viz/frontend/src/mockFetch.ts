// Mock fetch for UI preview — intercepts API calls and returns dummy data

const categories = ["Health", "Fitness", "Mood", "Sleep", "Productivity"];

const questions: Record<string, Array<{key: string; display_name: string; graph_type: string; is_positive: boolean; is_reverse: boolean; min_value: number; max_value: number; cadence: string}>> = {
  Health: [
    { key: "whoopHRV", display_name: "HRV", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 150, cadence: "daily" },
    { key: "whoopRHR", display_name: "Resting Heart Rate", graph_type: "line", is_positive: false, is_reverse: true, min_value: 40, max_value: 100, cadence: "daily" },
    { key: "whoopRecoveryScore", display_name: "Recovery Score", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 100, cadence: "daily" },
    { key: "whoopSpO2", display_name: "SpO2 %", graph_type: "line", is_positive: true, is_reverse: false, min_value: 90, max_value: 100, cadence: "daily" },
  ],
  Fitness: [
    { key: "whoopStrain", display_name: "Strain", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 21, cadence: "daily" },
    { key: "running", display_name: "Running", graph_type: "calendar", is_positive: true, is_reverse: false, min_value: 0, max_value: 1, cadence: "daily" },
    { key: "tricepCurl", display_name: "Tricep Curl", graph_type: "calendar", is_positive: true, is_reverse: false, min_value: 0, max_value: 1, cadence: "weekly" },
  ],
  Mood: [
    { key: "mood", display_name: "Mood", graph_type: "line", is_positive: true, is_reverse: false, min_value: 1, max_value: 5, cadence: "daily" },
    { key: "energy", display_name: "Energy", graph_type: "line", is_positive: true, is_reverse: false, min_value: 1, max_value: 5, cadence: "daily" },
    { key: "stress", display_name: "Stress", graph_type: "line", is_positive: false, is_reverse: true, min_value: 1, max_value: 5, cadence: "daily" },
  ],
  Sleep: [
    { key: "whoopSleepPerformance", display_name: "Sleep Performance", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 100, cadence: "daily" },
    { key: "whoopSleepEfficiency", display_name: "Sleep Efficiency", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 100, cadence: "daily" },
    { key: "sleptBefore1AM", display_name: "Slept Before 1AM", graph_type: "calendar", is_positive: true, is_reverse: false, min_value: 0, max_value: 1, cadence: "daily" },
  ],
  Productivity: [
    { key: "deepWork", display_name: "Deep Work", graph_type: "line", is_positive: true, is_reverse: false, min_value: 0, max_value: 8, cadence: "daily" },
    { key: "sugar", display_name: "Sugar", graph_type: "calendar", is_positive: false, is_reverse: true, min_value: 0, max_value: 1, cadence: "daily" },
    { key: "veggies", display_name: "Veggies & Fruits", graph_type: "calendar", is_positive: true, is_reverse: false, min_value: 0, max_value: 1, cadence: "daily" },
  ],
};

interface DataPoint { timestamp: string; value: number; }

function generateTimeSeries(min: number, max: number, days = 90): DataPoint[] {
  const result: DataPoint[] = [];
  const now = new Date();
  let val = (min + max) / 2;
  for (let i = days; i >= 0; i--) {
    const d = new Date(now);
    d.setDate(d.getDate() - i);
    val = Math.max(min, Math.min(max, val + (Math.random() - 0.48) * (max - min) * 0.12));
    if (Math.random() > 0.15) {
      result.push({ timestamp: d.toISOString(), value: Math.round(val * 10) / 10 });
    }
  }
  return result;
}

function generateCalendarData(days = 90): DataPoint[] {
  const result: DataPoint[] = [];
  const now = new Date();
  for (let i = days; i >= 0; i--) {
    const d = new Date(now);
    d.setDate(d.getDate() - i);
    if (Math.random() > 0.25) {
      result.push({ timestamp: d.toISOString(), value: Math.random() > 0.3 ? 1 : 0 });
    }
  }
  return result;
}

const originalFetch = window.fetch.bind(window);

export function installMockFetch() {
  if (process.env.REACT_APP_USE_MOCK !== 'true') return;
  window.fetch = async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
    const url = typeof input === 'string' ? input : input.toString();

    if (url.includes('/api/metadata')) {
      return mockResponse({ name: "Soumyadeep" });
    }

    if (url.includes('/api/categories')) {
      return mockResponse(categories.map(name => ({ name })));
    }

    if (url.includes('/api/questions')) {
      const match = url.match(/category=([^&]*)/);
      const cat = match ? decodeURIComponent(match[1]) : '';
      return mockResponse((questions[cat] || []).map((q, i) => ({ ...q, order: i })));
    }

    if (url.includes('/api/data/')) {
      const key = url.split('/api/data/')[1]?.split('?')[0];
      const allQ = Object.values(questions).flat();
      const q = allQ.find(x => x.key === key);
      const isCalendar = q?.graph_type === 'calendar';
      const data = isCalendar
        ? generateCalendarData()
        : generateTimeSeries(q?.min_value ?? 0, q?.max_value ?? 100);
      return mockResponse({ data });
    }

    return originalFetch(input, init);
  };
}

function mockResponse(data: unknown): Promise<Response> {
  return Promise.resolve(new Response(JSON.stringify(data), {
    status: 200,
    headers: { 'Content-Type': 'application/json' },
  }));
}

// Auto-install at module load when env var is set (side-effect import)
void (process.env.REACT_APP_USE_MOCK === 'true' && installMockFetch());
