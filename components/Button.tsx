import React from 'react';

interface ButtonProps {
    onClick: () => any;
    text: string;
}

const Button: React.FC<ButtonProps> = ({ onClick, text }) => {
    return (
        <button onClick={onClick} className="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600  hover:text-white  focus:ring-4 focus:outline-none focus:ring-blue-300 ">
<span className="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white  rounded-md group-hover:bg-opacity-0">
{text}
</span>
</button>
    );
};

export default Button;