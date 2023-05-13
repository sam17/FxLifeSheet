import React, { useEffect, useRef, useState } from 'react';
import styles from '../Tooltip.module.css';
import { getDateInString } from 'src/utils/date';

export interface tooltipData {
    visible: boolean;
    date : Date;
    value : string;
    isPositive : boolean;
}

interface TooltipProps {
    tooltipData: tooltipData;
}

const useMousePosition = () => {
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

  const updateMousePosition = (event: MouseEvent) => {
    setMousePosition({ x: event.pageX, y: event.pageY });
  };

  useEffect(() => {
    window.addEventListener('mousemove', updateMousePosition);

    return () => {
      window.removeEventListener('mousemove', updateMousePosition);
    };
  }, []);

  return mousePosition;
};

const getContentString = (tooltipData: tooltipData) => {
    const { date, value, isPositive } = tooltipData;
    const dateString = getDateInString(date);
    return `${value} </br> ${dateString}`;
}


const Tooltip: React.FC<TooltipProps> = ({ tooltipData }) => {
  const tooltipRef = useRef<HTMLDivElement | null>(null);
  const [position, setPosition] = useState({ left: 0, top: 0 });
  const mousePosition = useMousePosition();
  useEffect(() => {
    if (tooltipData.visible && tooltipRef.current) {
      const offset = 10; // Adjust this offset as needed

      const left = mousePosition.x + offset;
      const top = mousePosition.y -  offset;

      setPosition({
        left,
        top,
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

  const tooltipClass = `${styles.tooltip} ${tooltipData.isPositive ? styles.positiveTooltip : styles.negativeTooltip}`;

  return (
    <div ref={tooltipRef} className={tooltipClass} style={style}>
      <div dangerouslySetInnerHTML={{ __html: getContentString(tooltipData) }} />
    </div>
  );
};

export default Tooltip;