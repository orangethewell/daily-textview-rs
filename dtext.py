# imports
import datetime
from bs4 import BeautifulSoup
import urllib.request


# get page and content
numDate = datetime.datetime.now().strftime("%Y/%m/%d")
textPage = 'https://wol.jw.org/pt/wol/h/r5/lp-t/' + str(numDate)
html = urllib.request.urlopen(textPage).read().decode()
soup = BeautifulSoup(html, 'html.parser')


# get daily text components
dynDate = datetime.datetime.now().strftime("%A, %B %d")
verse = soup.find('p', attrs={'class': 'themeScrp'}).text.strip()
citation = soup.find('p', attrs={'class': 'sb'}).text.strip()


# sort all components
dailyText = str(dynDate + "\n\n" + verse + "\n\n" + citation + "\n")


# print
print (dailyText)
input ("Press any key to continue...")
