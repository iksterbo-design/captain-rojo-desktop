import React from "react";

export function Card({ title, children, className = "" }) {
  return (
    <div className={`bg-white rounded-lg border border-gray-200 p-5 ${className}`}>
      {title && <h3 className="text-lg font-semibold text-gray-800 mb-3">{title}</h3>}
      {children}
    </div>
  );
}
