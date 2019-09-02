import pdftotext

with open("/home/prhod/github/hachinohe-gtfs/rosenzu.pdf", "rb") as f:
    pdf = pdftotext.PDF(f)
    print(len(pdf))
    for page in pdf:
        print(page)
