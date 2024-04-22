#!/bin/bash
GREEN='\033[0;32m'
ORANGE='\033[0;33m'
NC='\033[0m' # No Color


echo -e "${GREEN}=> Installing system dependency${NC}"
sudo apt install openocd python3 python3-pip python3-venv

echo -e "\n${GREEN}=> Virtual env.${NC}"

echo -e "\n    ${ORANGE}Create virtual env${NC}"
python3 -m venv venv | sed 's/^/    /'

echo -e "\n${GREEN}=> Enter in venv${NC}"
source ./venv/bin/activate

echo -e "\n${GREEN}=> Python depencies${NC}"
pip install tk psutil

echo -e "\n    ${ORANGE}pip freeze${NC}"
pip freeze | sed 's/^/    /'

echo -e "\n${GREEN}=> Exit venv${NC}"
deactivate

echo -e "\n${GREEN}========= Done ! =========${NC}"
read -s -p "Press any key to close ..."
echo -e "\n\n"