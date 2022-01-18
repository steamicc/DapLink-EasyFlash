#!/bin/bash
GREEN='\033[0;32m'
ORANGE='\033[0;33m'
NC='\033[0m' # No Color


echo -e "${GREEN}=> Installing system dependency${NC}"
sudo apt install openocd python3 python3-pip python3-tk

echo -e "\n\n\n${GREEN}=> Virtual env.${NC}"
echo -e "\n\n    ${ORANGE}Installing virtualenv (pip)${NC}"
pip install virtualenv | sed 's/^/    /'

echo -e "\n\n    ${ORANGE}Create virtual env${NC}"
virtualenv venv | sed 's/^/    /'

echo -e "\n\n\n${GREEN}=> Enter in venv${NC}"
source ./venv/bin/activate

echo -e "\n\n\n${GREEN}=> Python depencies${NC}"
pip install pysimplegui psutil

echo -e "\n\n    ${ORANGE}pip freeze${NC}"
pip freeze | sed 's/^/    /'

echo -e "\n\n\n${GREEN}=> Exit venv${NC}"
deactivate

echo -e "\n\n\n${GREEN}========= Done ! =========${NC}"
read -s -p "Press any key to close ..."
echo -e "\n\n"