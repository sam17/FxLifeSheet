import React from 'react';
import styles from '../Tooltip.module.css';

interface TooltipProps {
  tooltipData: {
    visible: boolean;
    content: string;
  };
  position: {
    left: number;
    top: number;
  };
}

const Tooltip: React.FC<TooltipProps> = ({ tooltipData, position }) => {
  if (!tooltipData.visible) {
    return null;
  }

  const style = {
    left: position.left + 'px',
    top: position.top + 'px',
  };

  return (
    <div className={styles.tooltip} style={style}>
      <div dangerouslySetInnerHTML={{ __html: tooltipData.content }} />
    </div>
  );
};

export default Tooltip;