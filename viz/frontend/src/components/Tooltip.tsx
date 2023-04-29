import React, { useEffect, useRef, useState } from 'react';
import styles from '../Tooltip.module.css';

interface TooltipProps {
  tooltipData: {
    visible: boolean;
    content: string;
  };
}

const useMousePosition = () => {
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

  const updateMousePosition = (event: MouseEvent) => {
    setMousePosition({ x: event.clientX, y: event.clientY });
  };

  useEffect(() => {
    window.addEventListener('mousemove', updateMousePosition);

    return () => {
      window.removeEventListener('mousemove', updateMousePosition);
    };
  }, []);

  return mousePosition;
};

const Tooltip: React.FC<TooltipProps> = ({ tooltipData }) => {
  const tooltipRef = useRef<HTMLDivElement | null>(null);
  const [position, setPosition] = useState({ left: 0, top: 0 });
  const mousePosition = useMousePosition();

  useEffect(() => {
    if (tooltipData.visible && tooltipRef.current) {
      const tooltipWidth = tooltipRef.current.clientWidth;
      const tooltipHeight = tooltipRef.current.clientHeight;
      setPosition({
        left: mousePosition.x - tooltipWidth / 2,
        top: mousePosition.y - tooltipHeight - 10,
      });
    }
  }, [tooltipData.visible, mousePosition]);

  if (!tooltipData.visible) {
    return null;
  }

  const style = {
    left: position.left + 'px',
    top: position.top + 'px',
  };

  return (
    <div ref={tooltipRef} className={styles.tooltip} style={style}>
      <div dangerouslySetInnerHTML={{ __html: tooltipData.content }} />
    </div>
  );
};

export default Tooltip;