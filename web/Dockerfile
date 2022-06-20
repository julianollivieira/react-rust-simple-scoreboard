FROM ubuntu:21.10
WORKDIR /home/web

# Update and install some required packages.
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get -y install curl

# Install Node.js and Yarn.
RUN curl -sL https://deb.nodesource.com/setup_17.x | bash -
RUN apt-get -y install nodejs
RUN npm install --global yarn

# Install Node.js dependencies and run the web server.
RUN yarn install
CMD ["yarn", "dev", "--host"]