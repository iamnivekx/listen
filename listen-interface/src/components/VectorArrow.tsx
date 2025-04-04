import { useMobile } from "../contexts/MobileContext";

export const VectorArrow = () => {
  const { isVerySmallScreen } = useMobile();

  return (
    <svg
      width={isVerySmallScreen ? "14" : "16"}
      height={isVerySmallScreen ? "14" : "16"}
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M2.34315 7.375C1.99797 7.375 1.71815 7.65482 1.71815 8C1.71815 8.34518 1.99797 8.625 2.34315 8.625L2.34315 7.375ZM14.0988 8.44194C14.3429 8.19786 14.3429 7.80213 14.0988 7.55806L10.1213 3.58058C9.87724 3.3365 9.48151 3.3365 9.23744 3.58058C8.99336 3.82466 8.99336 4.22039 9.23744 4.46447L12.773 8L9.23744 11.5355C8.99336 11.7796 8.99336 12.1753 9.23744 12.4194C9.48152 12.6635 9.87724 12.6635 10.1213 12.4194L14.0988 8.44194ZM2.34315 8.625L13.6569 8.625L13.6569 7.375L2.34315 7.375L2.34315 8.625Z"
        fill="#D9D9D9"
      />
    </svg>
  );
};
