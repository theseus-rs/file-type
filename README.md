# File Type

[![ci](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/file_type/badge.svg)](https://docs.rs/file_type)
[![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
[![Latest version](https://img.shields.io/crates/v/file_type.svg)](https://crates.io/crates/file_type)
[![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

File Type is a library that uses file signatures and file extensions to determine the type of file.

Signatures, extensions and media type data are provided by:
* [The National Archives PRONOM](https://www.nationalarchives.gov.uk/pronom/)
* [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types)
* [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml)

## Getting Started

Detect a Java class file from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
assert_eq!(file_type.name(), "Java Class File");
assert_eq!(file_type.media_type(), Vec::<String>::new());
assert_eq!(file_type.extensions(), vec!["class"]);
```

Detect text from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"hello, world\n");
assert_eq!(file_type.name(), "Text");
assert_eq!(file_type.media_types(), vec!["text/plain"]);
assert_eq!(file_type.extensions(), Vec::<String>::new());
```

## Supported File Types

<!--FILE_TYPES_BEGIN-->
| | id | Name | Extensions | Media Types |
| ---- | ---- | ---- | ----------- | ---------- |
| 1 | linguist/0 | 1C Enterprise | .bsl, .os |  |
| 2 | linguist/387204628 | 2-Dimensional Array | .2da |  |
| 3 | x-fmt/19 | 3D Studio | 3ds |  |
| 4 | x-fmt/102 | 3D Studio (DOS) 2D Shape File | shp |  |
| 5 | fmt/1830 | 3D Studio (DOS) 2D/3D Loft Object File | lft |  |
| 6 | fmt/1831 | 3D Studio (DOS) Project File | prj |  |
| 7 | x-fmt/432 | 3DM | 3dm |  |
| 8 | x-fmt/434 | 3DM | 3dm |  |
| 9 | x-fmt/433 | 3DM | 3dm |  |
| 10 | x-fmt/435 | 3DM | 3dm |  |
| 11 | fmt/864 | 3DM | 3dm |  |
| 12 | fmt/978 | 3DS Max | max, chr |  |
| 13 | fmt/357 | 3GPP Audio/Video File | 3gp, 3gpp | audio/3gpp, video/3gpp |
| 14 | fmt/1275 | 3M Printscape | psc |  |
| 15 | fmt/829 | 3MF 3D Manufacturing Format | 3mf | application/vnd.ms-3mfdocument |
| 16 | linguist/577529595 | 4D | .4dm |  |
| 17 | fmt/1150 | 4X Movie File | 4xm, 4xa |  |
| 18 | fmt/1699 | 602 Graph/Chart File | gc6 |  |
| 19 | fmt/1774 | 602 Graph/Chart File | gc6 |  |
| 20 | fmt/1695 | 602 Text file | 602 |  |
| 21 | fmt/1294 | 602Tab Spreadsheet | wls |  |
| 22 | fmt/1293 | 602Text Document | wpd, wpt |  |
| 23 | x-fmt/21 | 7-bit ANSI Text | ans | text/plain |
| 24 | x-fmt/22 | 7-bit ASCII Text | asc | text/plain |
| 25 | fmt/484 | 7Zip format | 7z |  |
| 26 | x-fmt/282 | 8-bit ANSI Text | ans | text/plain |
| 27 | x-fmt/283 | 8-bit ASCII Text | asc | text/plain |
| 28 | fmt/980 | AAE Sidecar Format | aae |  |
| 29 | linguist/1 | ABAP | .abap |  |
| 30 | linguist/452681853 | ABAP CDS | .asddls |  |
| 31 | fmt/890 | AbiWord Document | abw |  |
| 32 | fmt/891 | AbiWord Document Template | awt |  |
| 33 | fmt/1463 | Ableton Live Set | als |  |
| 34 | linguist/429 | ABNF | .abnf |  |
| 35 | x-fmt/301 | ACBM Graphics | acb |  |
| 36 | fmt/1482 | Access Report Snapshot | snp |  |
| 37 | fmt/842 | AccessData Custom Content Image | ad1, ad2, ad3, ad4, ad5 |  |
| 38 | fmt/843 | AccessData Custom Content Image (Encrypted) | ad1, ad2, ad3, ad4, ad5 |  |
| 39 | fmt/452 | Acrobat Catalog Cat File | cat |  |
| 40 | x-fmt/427 | Acrobat Language definition file | lng |  |
| 41 | fmt/14 | Acrobat PDF 1.0 - Portable Document Format | pdf | application/pdf |
| 42 | fmt/15 | Acrobat PDF 1.1 - Portable Document Format | pdf | application/pdf |
| 43 | fmt/16 | Acrobat PDF 1.2 - Portable Document Format | pdf | application/pdf |
| 44 | fmt/17 | Acrobat PDF 1.3 - Portable Document Format | pdf | application/pdf |
| 45 | fmt/18 | Acrobat PDF 1.4 - Portable Document Format | pdf | application/pdf |
| 46 | fmt/19 | Acrobat PDF 1.5 - Portable Document Format | pdf | application/pdf |
| 47 | fmt/20 | Acrobat PDF 1.6 - Portable Document Format | pdf | application/pdf |
| 48 | fmt/276 | Acrobat PDF 1.7 - Portable Document Format | pdf | application/pdf |
| 49 | fmt/477 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 50 | fmt/480 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 51 | fmt/479 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 52 | fmt/354 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 53 | fmt/1910 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 54 | fmt/1911 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 55 | fmt/478 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 56 | fmt/476 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 57 | fmt/95 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 58 | fmt/481 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 59 | fmt/1912 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| 60 | fmt/493 | Acrobat PDF/E - Portable Document Format for Engineering PDF/E-1 | pdf | application/pdf |
| 61 | fmt/144 | Acrobat PDF/X - Portable Document Format - Exchange 1:1999 | pdf | application/pdf |
| 62 | fmt/145 | Acrobat PDF/X - Portable Document Format - Exchange 1:2001 | pdf | application/pdf |
| 63 | fmt/157 | Acrobat PDF/X - Portable Document Format - Exchange 1a:2001 | pdf | application/pdf |
| 64 | fmt/146 | Acrobat PDF/X - Portable Document Format - Exchange 1a:2003 | pdf | application/pdf |
| 65 | fmt/147 | Acrobat PDF/X - Portable Document Format - Exchange 2:2003 | pdf | application/pdf |
| 66 | fmt/158 | Acrobat PDF/X - Portable Document Format - Exchange 3:2002 | pdf | application/pdf |
| 67 | fmt/148 | Acrobat PDF/X - Portable Document Format - Exchange 3:2003 | pdf | application/pdf |
| 68 | fmt/488 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-4 | pdf | application/pdf |
| 69 | fmt/489 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-4p | pdf | application/pdf |
| 70 | fmt/490 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5g | pdf | application/pdf |
| 71 | fmt/492 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5n | pdf | application/pdf |
| 72 | fmt/491 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5pg | pdf | application/pdf |
| 73 | linguist/10 | ActionScript | .as |  |
| 74 | x-fmt/138 | Active Server Page | asp |  |
| 75 | fmt/1915 | ActiveMime Object | mso |  |
| 76 | fmt/498 | ActiveX License Package file | lpk |  |
| 77 | linguist/11 | Ada | .adb, .ada, .ads |  |
| 78 | fmt/356 | Adaptive Multi-Rate Audio | amr | audio/amr |
| 79 | fmt/954 | Adaptive Multi-Rate Wideband Audio | awb | audio/amr-wb |
| 80 | linguist/884614762 | Adblock Filter List | .txt |  |
| 81 | fmt/697 | Additive Manufacturing File Format | amf |  |
| 82 | x-fmt/217 | Adobe ACD | acd |  |
| 83 | fmt/1500 | Adobe Acrobat Forms Data Format | fdf | application/vnd.fdf |
| 84 | fmt/796 | Adobe After Effects | aep |  |
| 85 | fmt/943 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| 86 | fmt/937 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| 87 | fmt/942 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| 88 | fmt/1859 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| 89 | fmt/1053 | Adobe Audio Waveform | pek |  |
| 90 | fmt/1499 | Adobe Audition Session File | sesx |  |
| 91 | fmt/1814 | Adobe Color Book for Windows | acb |  |
| 92 | fmt/1815 | Adobe Color Swatch | aco |  |
| 93 | fmt/871 | Adobe Content Server Message File | acsm | application/vnd.adobe.adept+xml |
| 94 | fmt/767 | Adobe Flash | swf | application/x-shockwave-flash |
| 95 | fmt/772 | Adobe Flash | swf | application/x-shockwave-flash |
| 96 | fmt/761 | Adobe Flash | swf | application/x-shockwave-flash |
| 97 | fmt/762 | Adobe Flash | swf | application/x-shockwave-flash |
| 98 | fmt/507 | Adobe Flash | swf | application/x-shockwave-flash |
| 99 | fmt/771 | Adobe Flash | swf | application/x-shockwave-flash |
| 100 | fmt/773 | Adobe Flash | swf | application/x-shockwave-flash |
| 101 | fmt/758 | Adobe Flash | swf | application/x-shockwave-flash |
| 102 | fmt/506 | Adobe Flash | swf | application/x-shockwave-flash |
| 103 | fmt/769 | Adobe Flash | swf | application/x-shockwave-flash |
| 104 | fmt/770 | Adobe Flash | swf | application/x-shockwave-flash |
| 105 | fmt/765 | Adobe Flash | swf | application/x-shockwave-flash |
| 106 | fmt/774 | Adobe Flash | swf | application/x-shockwave-flash |
| 107 | fmt/768 | Adobe Flash | swf | application/x-shockwave-flash |
| 108 | fmt/764 | Adobe Flash | swf | application/x-shockwave-flash |
| 109 | fmt/776 | Adobe Flash | swf | application/x-shockwave-flash |
| 110 | fmt/775 | Adobe Flash | swf | application/x-shockwave-flash |
| 111 | fmt/757 | Adobe Flash | swf | application/x-shockwave-flash |
| 112 | fmt/759 | Adobe Flash | swf | application/x-shockwave-flash |
| 113 | fmt/763 | Adobe Flash | swf | application/x-shockwave-flash |
| 114 | fmt/766 | Adobe Flash | swf | application/x-shockwave-flash |
| 115 | fmt/760 | Adobe Flash | swf | application/x-shockwave-flash |
| 116 | fmt/505 | Adobe Flash | swf | application/x-shockwave-flash |
| 117 | fmt/526 | Adobe Font List | lst |  |
| 118 | linguist/147198098 | Adobe Font Metrics | .afm |  |
| 119 | fmt/534 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 120 | fmt/536 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 121 | fmt/537 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 122 | fmt/533 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 123 | fmt/539 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 124 | fmt/190 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 125 | fmt/538 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 126 | x-fmt/302 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 127 | fmt/535 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| 128 | x-fmt/162 | Adobe FrameMaker Interchange Format | mif | application/vnd.mif |
| 129 | fmt/563 | Adobe Illustrator | ai, pdf | application/postscript |
| 130 | fmt/557 | Adobe Illustrator | ai, eps | application/postscript |
| 131 | fmt/418 | Adobe Illustrator | ai | application/postscript |
| 132 | fmt/421 | Adobe Illustrator | ai | application/postscript |
| 133 | x-fmt/20 | Adobe Illustrator | ai | application/postscript |
| 134 | fmt/564 | Adobe Illustrator | ai, pdf | application/postscript |
| 135 | fmt/559 | Adobe Illustrator | ai, pdf | application/postscript |
| 136 | fmt/419 | Adobe Illustrator | ai | application/postscript |
| 137 | fmt/420 | Adobe Illustrator | ai | application/postscript |
| 138 | fmt/417 | Adobe Illustrator | ai | application/postscript |
| 139 | fmt/560 | Adobe Illustrator | ai, pdf | application/postscript |
| 140 | fmt/423 | Adobe Illustrator | ai | application/postscript |
| 141 | fmt/561 | Adobe Illustrator | ai, pdf | application/postscript |
| 142 | fmt/565 | Adobe Illustrator | ai, pdf | application/postscript |
| 143 | fmt/558 | Adobe Illustrator | ai, pdf | application/postscript |
| 144 | fmt/422 | Adobe Illustrator | ai, eps | application/postscript |
| 145 | fmt/562 | Adobe Illustrator | ai, pdf | application/postscript |
| 146 | fmt/1863 | Adobe Illustrator CC 2020 Artwork | ai, ait |  |
| 147 | fmt/1864 | Adobe Illustrator CC 2020 Artwork | ai, ait |  |
| 148 | fmt/1862 | Adobe Illustrator CC Artwork | ai, ait |  |
| 149 | fmt/1191 | Adobe InDesign Book | indb |  |
| 150 | fmt/1634 | Adobe InDesign Document | indd, ind, indt |  |
| 151 | fmt/550 | Adobe InDesign Document | ind, indd, indt |  |
| 152 | fmt/1942 | Adobe InDesign Document | indd, ind, indt |  |
| 153 | fmt/1630 | Adobe InDesign Document | indd, ind, indt |  |
| 154 | fmt/552 | Adobe InDesign Document | ind, indd, indt |  |
| 155 | fmt/1633 | Adobe InDesign Document | indd, ind, indt |  |
| 156 | fmt/548 | Adobe InDesign Document | ind, indd, indt |  |
| 157 | fmt/1628 | Adobe InDesign Document | indd, ind |  |
| 158 | fmt/1638 | Adobe InDesign Document | indd, ind, indt |  |
| 159 | fmt/1631 | Adobe InDesign Document | indd, ind, indt |  |
| 160 | x-fmt/450 | Adobe InDesign Document | ind, indd, indt | application/octet-stream |
| 161 | fmt/1637 | Adobe InDesign Document | indd, ind, indt |  |
| 162 | fmt/1639 | Adobe InDesign Document | indd, ind, indt |  |
| 163 | fmt/1632 | Adobe InDesign Document | indd, ind, indt |  |
| 164 | fmt/549 | Adobe InDesign Document | ind, indd, indt |  |
| 165 | fmt/551 | Adobe InDesign Document | ind, indd, indt |  |
| 166 | fmt/1635 | Adobe InDesign Document | indd, ind, indt |  |
| 167 | fmt/1941 | Adobe InDesign Document | indd, ind, indt |  |
| 168 | fmt/1636 | Adobe InDesign Document | indd, ind, indt |  |
| 169 | fmt/1629 | Adobe InDesign Document | indd, ind, indt |  |
| 170 | fmt/1640 | Adobe InDesign Document | indd, ind, indt |  |
| 171 | fmt/196 | Adobe InDesign Document | indd, ind, indt |  |
| 172 | fmt/1641 | Adobe InDesign Interchange Document | inx |  |
| 173 | fmt/1642 | Adobe InDesign Library | indl |  |
| 174 | fmt/1192 | Adobe InDesign Library | indl |  |
| 175 | fmt/521 | Adobe Multiple Master Metrics font file | mmm |  |
| 176 | x-fmt/167 | Adobe PhotoDeluxe | pdd |  |
| 177 | x-fmt/92 | Adobe Photoshop | psd, pdd | image/vnd.adobe.photoshop |
| 178 | fmt/996 | Adobe Photoshop Large Document Format | psb | image/vnd.adobe.photoshop |
| 179 | fmt/449 | Adobe Portable Document Catalog Index File | pdx |  |
| 180 | fmt/447 | Adobe Portable Document Catalog Index File | pdx |  |
| 181 | fmt/446 | Adobe Portable Document Catalog Index File | pdx |  |
| 182 | fmt/448 | Adobe Portable Document Catalog Index File | pdx |  |
| 183 | fmt/509 | Adobe PostScript Font Metrics file | pfm |  |
| 184 | fmt/525 | Adobe Printer Font Binary | pfb |  |
| 185 | fmt/1816 | Adobe Swatch Exchange | ase |  |
| 186 | fmt/1190 | Adobe SWC Package | swc |  |
| 187 | fmt/660 | Adobe Type 1 Mac Font File |  |  |
| 188 | fmt/1584 | ADRIFT Text Adventure File | taf |  |
| 189 | fmt/1370 | Advanced Disk Catalog | adc |  |
| 190 | fmt/844 | Advanced Forensic Format | aff |  |
| 191 | fmt/683 | Advanced Function Presentation | afp |  |
| 192 | fmt/131 | Advanced Systems Format | asf | application/vnd.ms-asf |
| 193 | fmt/840 | ADX Audio Format | adx |  |
| 194 | fmt/1620 | Aero Studio Song | aero |  |
| 195 | linguist/12 | Agda | .agda |  |
| 196 | fmt/1505 | Agisoft Point Cloud | oc3 |  |
| 197 | fmt/1502 | Agisoft Project Archive | psz |  |
| 198 | fmt/1503 | Agisoft Project File | psx |  |
| 199 | fmt/1504 | Agisoft Tiled Model | tls |  |
| 200 | fmt/1649 | AGS 4 Data Format | ags |  |
| 201 | linguist/2 | AGS Script | .asc, .ash | text/x-c++src |
| 202 | fmt/1621 | AHX-Module Format (formerly THX module format) | ahx |  |
| 203 | linguist/451700185 | AIDL | .aidl |  |
| 204 | linguist/658971832 | AL | .al |  |
| 205 | fmt/1449 | Aldus FreeHand Drawing |  |  |
| 206 | x-fmt/303 | Aldus Freehand Drawing | fh3 |  |
| 207 | x-fmt/304 | Aldus Freehand Drawing | fh4 |  |
| 208 | fmt/1450 | Aldus FreeHand Drawing |  |  |
| 209 | fmt/1092 | Alias Pix Image File | pix, ico |  |
| 210 | fmt/1171 | Alias PowerAnimator File |  |  |
| 211 | fmt/1093 | Alias Scene Description Language | sdl |  |
| 212 | fmt/1175 | Alias Studio Wire File |  |  |
| 213 | fmt/1170 | Alias Studio Wire File |  |  |
| 214 | linguist/13 | Alloy | .als |  |
| 215 | linguist/14 | Alpine Abuild |  | text/x-sh |
| 216 | linguist/187772328 | Altium Designer | .OutJob, .PcbDoc, .PrjPCB, .SchDoc |  |
| 217 | fmt/1937 | Amazon Kindle eBook File | azw, azw3, mobi, amr |  |
| 218 | x-fmt/290 | AMI Draw Vector Image | sdw |  |
| 219 | x-fmt/191 | AMI Professional Document | sam | application/vnd.lotus-wordpro |
| 220 | fmt/1361 | Amiga Disk File | adf |  |
| 221 | fmt/918 | AmiraMesh | am, amiramesh, hx |  |
| 222 | fmt/917 | AmiraMesh | am, amiramesh, hx |  |
| 223 | fmt/921 | AmiraMesh | am, amiramesh, hx |  |
| 224 | fmt/919 | AmiraMesh | am, amiramesh, hx |  |
| 225 | fmt/920 | AmiraMesh | am, amiramesh, hx |  |
| 226 | linguist/3 | AMPL | .ampl, .mod |  |
| 227 | linguist/389477596 | AngelScript | .as, .angelscript | text/x-c++src |
| 228 | fmt/935 | Animated Portable Network Graphics | png, apng | image/vnd.mozilla.apng |
| 229 | fmt/1784 | Animatic Film Format | flm |  |
| 230 | linguist/15 | Ant Build System |  | application/xml |
| 231 | linguist/1067292663 | Antlers | .antlers.html, .antlers.php, .antlers.xml |  |
| 232 | linguist/4 | ANTLR | .g4 |  |
| 233 | linguist/16 | ApacheConf | .apacheconf, .vhost |  |
| 234 | linguist/17 | Apex | .cls, .trigger | text/x-java |
| 235 | linguist/5 | API Blueprint | .apib |  |
| 236 | linguist/6 | APL | .apl, .dyalog | text/apl |
| 237 | linguist/18 | Apollo Guidance Computer | .agc |  |
| 238 | fmt/416 | Apple Core Audio Format | caf |  |
| 239 | fmt/625 | Apple Disk Copy Image | dmg, smi, img, image |  |
| 240 | fmt/1071 | Apple Disk Image | dmg | application/x-apple-diskimage |
| 241 | fmt/482 | Apple iBook format | ibooks | application/x-ibooks+zip |
| 242 | fmt/1185 | Apple Icon Image Format | icns |  |
| 243 | fmt/1441 | Apple iWork Document | iwa, key, pages, numbers, template |  |
| 244 | fmt/646 | Apple iWork Keynote | key |  |
| 245 | fmt/1440 | Apple iWork Numbers |  |  |
| 246 | fmt/825 | Apple iWork Numbers | numbers |  |
| 247 | fmt/1439 | Apple iWork Pages | pages |  |
| 248 | fmt/824 | Apple iWork Pages | pages |  |
| 249 | fmt/1187 | Apple iWork Template | template |  |
| 250 | fmt/596 | Apple Lossless Audio Codec | m4a, mp4 |  |
| 251 | fmt/1757 | Apple Partition Map - ISO 9660 - UDF Hybrid Disk Image | iso, toast, dmg |  |
| 252 | fmt/1740 | Apple Partition Map Disk Image | toast, iso, cdr, dmg, bin, img |  |
| 253 | fmt/1741 | Apple Partition Map ISO 9660 Hybrid | toast, iso, cdr |  |
| 254 | fmt/797 | Apple ProRes | mov |  |
| 255 | fmt/866 | Apple Safari Webarchive | webarchive |  |
| 256 | x-fmt/305 | Apple Sound | afc |  |
| 257 | fmt/503 | AppleDouble Resource Fork |  | multipart/appledouble |
| 258 | fmt/966 | AppleDouble Resource Fork |  | multipart/appledouble |
| 259 | linguist/19 | AppleScript | .applescript, .scpt |  |
| 260 | fmt/968 | AppleSingle | as | application/applefile |
| 261 | fmt/967 | AppleSingle | as | application/applefile |
| 262 | fmt/1715 | Applet Effect Factory Config File | data |  |
| 263 | fmt/751 | AppleWorks Database | cwk |  |
| 264 | fmt/748 | AppleWorks Drawing | cwk |  |
| 265 | fmt/752 | AppleWorks Painting | cwk |  |
| 266 | fmt/753 | AppleWorks Presentation | cwk |  |
| 267 | fmt/750 | AppleWorks Spreadsheet | cwk |  |
| 268 | fmt/749 | AppleWorks Word Processor | cwk |  |
| 269 | apache-httpd/17267421299241784478 | application/andrew-inset | ez | application/andrew-inset |
| 270 | apache-httpd/9816146314707347696 | application/applixware | aw | application/applixware |
| 271 | apache-httpd/3641799221620537973 | application/atom+xml | atom | application/atom+xml |
| 272 | apache-httpd/2088900736017873143 | application/atomcat+xml | atomcat | application/atomcat+xml |
| 273 | apache-httpd/13485419110853384641 | application/atomsvc+xml | atomsvc | application/atomsvc+xml |
| 274 | apache-httpd/7834014297002784124 | application/ccxml+xml | ccxml | application/ccxml+xml |
| 275 | apache-httpd/5838795347239654995 | application/cdmi-capability | cdmia | application/cdmi-capability |
| 276 | apache-httpd/5776032609182544687 | application/cdmi-container | cdmic | application/cdmi-container |
| 277 | apache-httpd/6048803440915610366 | application/cdmi-domain | cdmid | application/cdmi-domain |
| 278 | apache-httpd/2596188386429928103 | application/cdmi-object | cdmio | application/cdmi-object |
| 279 | apache-httpd/7140578712814967016 | application/cdmi-queue | cdmiq | application/cdmi-queue |
| 280 | apache-httpd/12846344509288362340 | application/cu-seeme | cu | application/cu-seeme |
| 281 | apache-httpd/10465296028727937591 | application/davmount+xml | davmount | application/davmount+xml |
| 282 | apache-httpd/6389577377495108203 | application/docbook+xml | dbk | application/docbook+xml |
| 283 | apache-httpd/13430231427750105170 | application/dssc+der | dssc | application/dssc+der |
| 284 | apache-httpd/10674547702623898054 | application/dssc+xml | xdssc | application/dssc+xml |
| 285 | apache-httpd/5284432839487308324 | application/ecmascript | ecma | application/ecmascript |
| 286 | apache-httpd/759975750900967096 | application/emma+xml | emma | application/emma+xml |
| 287 | apache-httpd/3404069067379740569 | application/exi | exi | application/exi |
| 288 | apache-httpd/6014088398110303029 | application/font-tdpfr | pfr | application/font-tdpfr |
| 289 | apache-httpd/15669081077453213107 | application/gpx+xml | gpx | application/gpx+xml |
| 290 | apache-httpd/4297062800028961418 | application/gxf | gxf | application/gxf |
| 291 | apache-httpd/2482147210414887812 | application/hyperstudio | stk | application/hyperstudio |
| 292 | apache-httpd/7965017775226841149 | application/inkml+xml | ink, inkml | application/inkml+xml |
| 293 | apache-httpd/799891437585884025 | application/ipfix | ipfix | application/ipfix |
| 294 | apache-httpd/18408904452883758494 | application/java-serialized-object | ser | application/java-serialized-object |
| 295 | apache-httpd/16922485787373200833 | application/java-vm | class | application/java-vm |
| 296 | apache-httpd/8812856811677659676 | application/jsonml+json | jsonml | application/jsonml+json |
| 297 | apache-httpd/5927135927038744495 | application/lost+xml | lostxml | application/lost+xml |
| 298 | apache-httpd/7253703184866025307 | application/mac-compactpro | cpt | application/mac-compactpro |
| 299 | apache-httpd/13249549495329969252 | application/mads+xml | mads | application/mads+xml |
| 300 | apache-httpd/15435368578074645444 | application/marc | mrc | application/marc |
| 301 | apache-httpd/1512096571599294824 | application/marcxml+xml | mrcx | application/marcxml+xml |
| 302 | apache-httpd/3699261783011718340 | application/mathml+xml | mathml | application/mathml+xml |
| 303 | apache-httpd/5549479812584870908 | application/mediaservercontrol+xml | mscml | application/mediaservercontrol+xml |
| 304 | apache-httpd/7305870426760626928 | application/metalink+xml | metalink | application/metalink+xml |
| 305 | apache-httpd/2965419729449071055 | application/metalink4+xml | meta4 | application/metalink4+xml |
| 306 | apache-httpd/16965387237143203176 | application/mets+xml | mets | application/mets+xml |
| 307 | apache-httpd/6423878811909784008 | application/mods+xml | mods | application/mods+xml |
| 308 | apache-httpd/3672696470516843498 | application/mp21 | m21, mp21 | application/mp21 |
| 309 | apache-httpd/3013817452668215055 | application/mp4 | mp4s | application/mp4 |
| 310 | apache-httpd/16088873498976028686 | application/octet-stream | bin, dms, lrf, mar, so, dist, distz, pkg, bpk, dump, elc, deploy | application/octet-stream |
| 311 | apache-httpd/15645786132478526949 | application/oda | oda | application/oda |
| 312 | apache-httpd/8488995837589784861 | application/oebps-package+xml | opf | application/oebps-package+xml |
| 313 | apache-httpd/701141407755406764 | application/ogg | ogx | application/ogg |
| 314 | apache-httpd/15288519035999881865 | application/omdoc+xml | omdoc | application/omdoc+xml |
| 315 | apache-httpd/13615448736560673113 | application/onenote | onetoc, onetoc2, onetmp, onepkg | application/onenote |
| 316 | apache-httpd/11260398242129167003 | application/patch-ops-error+xml | xer | application/patch-ops-error+xml |
| 317 | apache-httpd/13565162211660515468 | application/pgp-encrypted | pgp | application/pgp-encrypted |
| 318 | apache-httpd/10735678142131287808 | application/pgp-signature | asc, sig | application/pgp-signature |
| 319 | apache-httpd/13788567070806270645 | application/pics-rules | prf | application/pics-rules |
| 320 | apache-httpd/147689487279366576 | application/pkcs10 | p10 | application/pkcs10 |
| 321 | apache-httpd/8480341370012650784 | application/pkcs8 | p8 | application/pkcs8 |
| 322 | apache-httpd/4889268195384677215 | application/pkix-attr-cert | ac | application/pkix-attr-cert |
| 323 | apache-httpd/13433049222230317362 | application/pkix-cert | cer | application/pkix-cert |
| 324 | apache-httpd/17267006425445237121 | application/pkix-crl | crl | application/pkix-crl |
| 325 | apache-httpd/6318308104775159433 | application/pkix-pkipath | pkipath | application/pkix-pkipath |
| 326 | apache-httpd/992198684389833801 | application/pkixcmp | pki | application/pkixcmp |
| 327 | apache-httpd/8533502177668954884 | application/pls+xml | pls | application/pls+xml |
| 328 | apache-httpd/4518020368328841134 | application/prs.cww | cww | application/prs.cww |
| 329 | apache-httpd/17622281248786972968 | application/pskc+xml | pskcxml | application/pskc+xml |
| 330 | apache-httpd/47924783087979020 | application/reginfo+xml | rif | application/reginfo+xml |
| 331 | apache-httpd/5054915061890795333 | application/relax-ng-compact-syntax | rnc | application/relax-ng-compact-syntax |
| 332 | apache-httpd/47530549795300406 | application/resource-lists+xml | rl | application/resource-lists+xml |
| 333 | apache-httpd/6901913721037133475 | application/resource-lists-diff+xml | rld | application/resource-lists-diff+xml |
| 334 | apache-httpd/9575045232419729220 | application/rls-services+xml | rs | application/rls-services+xml |
| 335 | apache-httpd/12296210021193523472 | application/rpki-ghostbusters | gbr | application/rpki-ghostbusters |
| 336 | apache-httpd/16270552558721564289 | application/rpki-manifest | mft | application/rpki-manifest |
| 337 | apache-httpd/3224209250085143021 | application/rpki-roa | roa | application/rpki-roa |
| 338 | apache-httpd/5789844385051418491 | application/rsd+xml | rsd | application/rsd+xml |
| 339 | apache-httpd/12753032465493873976 | application/rss+xml | rss | application/rss+xml |
| 340 | apache-httpd/7633132484611881254 | application/sbml+xml | sbml | application/sbml+xml |
| 341 | apache-httpd/1489417600909014564 | application/scvp-cv-request | scq | application/scvp-cv-request |
| 342 | apache-httpd/17692487408723072806 | application/scvp-cv-response | scs | application/scvp-cv-response |
| 343 | apache-httpd/16843356083413255425 | application/scvp-vp-request | spq | application/scvp-vp-request |
| 344 | apache-httpd/3800111246559995671 | application/scvp-vp-response | spp | application/scvp-vp-response |
| 345 | apache-httpd/12792425945248664173 | application/sdp | sdp | application/sdp |
| 346 | apache-httpd/10045809873570187998 | application/set-payment-initiation | setpay | application/set-payment-initiation |
| 347 | apache-httpd/3148081406512757481 | application/set-registration-initiation | setreg | application/set-registration-initiation |
| 348 | apache-httpd/12821240876237287409 | application/shf+xml | shf | application/shf+xml |
| 349 | apache-httpd/5137819976307855937 | application/smil+xml | smi, smil | application/smil+xml |
| 350 | apache-httpd/11948227751138753910 | application/sparql-query | rq | application/sparql-query |
| 351 | apache-httpd/3123908653018764713 | application/sparql-results+xml | srx | application/sparql-results+xml |
| 352 | apache-httpd/10097865076610039677 | application/srgs | gram | application/srgs |
| 353 | apache-httpd/593596079114199361 | application/srgs+xml | grxml | application/srgs+xml |
| 354 | apache-httpd/17814697578080827582 | application/sru+xml | sru | application/sru+xml |
| 355 | apache-httpd/8296049173261629556 | application/ssdl+xml | ssdl | application/ssdl+xml |
| 356 | apache-httpd/18215989259559031662 | application/ssml+xml | ssml | application/ssml+xml |
| 357 | apache-httpd/16755885106588162713 | application/thraud+xml | tfi | application/thraud+xml |
| 358 | apache-httpd/1889659990446513902 | application/timestamped-data | tsd | application/timestamped-data |
| 359 | apache-httpd/15983373516404118760 | application/vnd.3gpp.pic-bw-large | plb | application/vnd.3gpp.pic-bw-large |
| 360 | apache-httpd/3513807239393968462 | application/vnd.3gpp.pic-bw-small | psb | application/vnd.3gpp.pic-bw-small |
| 361 | apache-httpd/11384243993817082359 | application/vnd.3gpp.pic-bw-var | pvb | application/vnd.3gpp.pic-bw-var |
| 362 | apache-httpd/7654423572304751873 | application/vnd.3gpp2.tcap | tcap | application/vnd.3gpp2.tcap |
| 363 | apache-httpd/14648864362426505394 | application/vnd.3m.post-it-notes | pwn | application/vnd.3m.post-it-notes |
| 364 | apache-httpd/16487376073977232996 | application/vnd.accpac.simply.aso | aso | application/vnd.accpac.simply.aso |
| 365 | apache-httpd/16508893075482541318 | application/vnd.accpac.simply.imp | imp | application/vnd.accpac.simply.imp |
| 366 | apache-httpd/7113264481099194993 | application/vnd.acucobol | acu | application/vnd.acucobol |
| 367 | apache-httpd/11344963921236519991 | application/vnd.acucorp | atc, acutc | application/vnd.acucorp |
| 368 | apache-httpd/7116538921685197509 | application/vnd.adobe.formscentral.fcdt | fcdt | application/vnd.adobe.formscentral.fcdt |
| 369 | apache-httpd/2260535582776616 | application/vnd.adobe.fxp | fxp, fxpl | application/vnd.adobe.fxp |
| 370 | apache-httpd/16997561686559408896 | application/vnd.adobe.xdp+xml | xdp | application/vnd.adobe.xdp+xml |
| 371 | apache-httpd/4986673632147286331 | application/vnd.ahead.space | ahead | application/vnd.ahead.space |
| 372 | apache-httpd/4165121783115442141 | application/vnd.airzip.filesecure.azf | azf | application/vnd.airzip.filesecure.azf |
| 373 | apache-httpd/9209038509344480468 | application/vnd.airzip.filesecure.azs | azs | application/vnd.airzip.filesecure.azs |
| 374 | apache-httpd/3660531883732386782 | application/vnd.amazon.ebook | azw | application/vnd.amazon.ebook |
| 375 | apache-httpd/11880520770616751632 | application/vnd.americandynamics.acc | acc | application/vnd.americandynamics.acc |
| 376 | apache-httpd/18255916549466049934 | application/vnd.amiga.ami | ami | application/vnd.amiga.ami |
| 377 | apache-httpd/11490512282215503801 | application/vnd.android.package-archive | apk | application/vnd.android.package-archive |
| 378 | apache-httpd/596873008912409075 | application/vnd.anser-web-certificate-issue-initiation | cii | application/vnd.anser-web-certificate-issue-initiation |
| 379 | apache-httpd/8243032856616937357 | application/vnd.anser-web-funds-transfer-initiation | fti | application/vnd.anser-web-funds-transfer-initiation |
| 380 | apache-httpd/858406622269959928 | application/vnd.antix.game-component | atx | application/vnd.antix.game-component |
| 381 | apache-httpd/1006546886522380100 | application/vnd.apple.installer+xml | mpkg | application/vnd.apple.installer+xml |
| 382 | apache-httpd/9044968354667619340 | application/vnd.apple.mpegurl | m3u8 | application/vnd.apple.mpegurl |
| 383 | apache-httpd/16752839964839226118 | application/vnd.aristanetworks.swi | swi | application/vnd.aristanetworks.swi |
| 384 | apache-httpd/6375177958950997398 | application/vnd.astraea-software.iota | iota | application/vnd.astraea-software.iota |
| 385 | apache-httpd/13429870797822209580 | application/vnd.audiograph | aep | application/vnd.audiograph |
| 386 | apache-httpd/2469893521022682751 | application/vnd.blueice.multipass | mpm | application/vnd.blueice.multipass |
| 387 | apache-httpd/5904094350807371736 | application/vnd.bmi | bmi | application/vnd.bmi |
| 388 | apache-httpd/9491002127237071349 | application/vnd.businessobjects | rep | application/vnd.businessobjects |
| 389 | apache-httpd/13641015261737520885 | application/vnd.chemdraw+xml | cdxml | application/vnd.chemdraw+xml |
| 390 | apache-httpd/14672958157522374745 | application/vnd.chipnuts.karaoke-mmd | mmd | application/vnd.chipnuts.karaoke-mmd |
| 391 | apache-httpd/10930648055820933480 | application/vnd.cinderella | cdy | application/vnd.cinderella |
| 392 | apache-httpd/6703499962162694527 | application/vnd.claymore | cla | application/vnd.claymore |
| 393 | apache-httpd/4161863740758171848 | application/vnd.cloanto.rp9 | rp9 | application/vnd.cloanto.rp9 |
| 394 | apache-httpd/13373727545469518801 | application/vnd.clonk.c4group | c4g, c4d, c4f, c4p, c4u | application/vnd.clonk.c4group |
| 395 | apache-httpd/2672622462468315941 | application/vnd.cluetrust.cartomobile-config | c11amc | application/vnd.cluetrust.cartomobile-config |
| 396 | apache-httpd/5331962339813467974 | application/vnd.cluetrust.cartomobile-config-pkg | c11amz | application/vnd.cluetrust.cartomobile-config-pkg |
| 397 | apache-httpd/13251929814084281596 | application/vnd.commonspace | csp | application/vnd.commonspace |
| 398 | apache-httpd/17784298339216295779 | application/vnd.contact.cmsg | cdbcmsg | application/vnd.contact.cmsg |
| 399 | apache-httpd/13844007418067567392 | application/vnd.cosmocaller | cmc | application/vnd.cosmocaller |
| 400 | apache-httpd/16268303164982887344 | application/vnd.crick.clicker | clkx | application/vnd.crick.clicker |
| 401 | apache-httpd/17280861777505608854 | application/vnd.crick.clicker.keyboard | clkk | application/vnd.crick.clicker.keyboard |
| 402 | apache-httpd/2028789183123440400 | application/vnd.crick.clicker.palette | clkp | application/vnd.crick.clicker.palette |
| 403 | apache-httpd/13800072163877662241 | application/vnd.crick.clicker.template | clkt | application/vnd.crick.clicker.template |
| 404 | apache-httpd/18057261898033090654 | application/vnd.crick.clicker.wordbank | clkw | application/vnd.crick.clicker.wordbank |
| 405 | apache-httpd/4179771948179943531 | application/vnd.criticaltools.wbs+xml | wbs | application/vnd.criticaltools.wbs+xml |
| 406 | apache-httpd/16761428806224039353 | application/vnd.ctc-posml | pml | application/vnd.ctc-posml |
| 407 | apache-httpd/13469738755405299163 | application/vnd.cups-ppd | ppd | application/vnd.cups-ppd |
| 408 | apache-httpd/10354148540362929487 | application/vnd.curl.car | car | application/vnd.curl.car |
| 409 | apache-httpd/13037335205551783511 | application/vnd.curl.pcurl | pcurl | application/vnd.curl.pcurl |
| 410 | apache-httpd/5541150871595351962 | application/vnd.dart | dart | application/vnd.dart |
| 411 | apache-httpd/18329094975064823008 | application/vnd.data-vision.rdz | rdz | application/vnd.data-vision.rdz |
| 412 | apache-httpd/16059951786718315147 | application/vnd.dece.data | uvf, uvvf, uvd, uvvd | application/vnd.dece.data |
| 413 | apache-httpd/619002040759349425 | application/vnd.dece.ttml+xml | uvt, uvvt | application/vnd.dece.ttml+xml |
| 414 | apache-httpd/5230894226506666425 | application/vnd.dece.unspecified | uvx, uvvx | application/vnd.dece.unspecified |
| 415 | apache-httpd/12792181662701773000 | application/vnd.dece.zip | uvz, uvvz | application/vnd.dece.zip |
| 416 | apache-httpd/14103138331119806028 | application/vnd.denovo.fcselayout-link | fe_launch | application/vnd.denovo.fcselayout-link |
| 417 | apache-httpd/3054944743288342147 | application/vnd.dna | dna | application/vnd.dna |
| 418 | apache-httpd/16819287819186782796 | application/vnd.dolby.mlp | mlp | application/vnd.dolby.mlp |
| 419 | apache-httpd/6100025454960806313 | application/vnd.dpgraph | dpg | application/vnd.dpgraph |
| 420 | apache-httpd/2476393985808283697 | application/vnd.dreamfactory | dfac | application/vnd.dreamfactory |
| 421 | apache-httpd/7052973897628710533 | application/vnd.ds-keypoint | kpxx | application/vnd.ds-keypoint |
| 422 | apache-httpd/5012686256345022475 | application/vnd.dvb.ait | ait | application/vnd.dvb.ait |
| 423 | apache-httpd/14648134999256152995 | application/vnd.dvb.service | svc | application/vnd.dvb.service |
| 424 | apache-httpd/14560088302329463820 | application/vnd.dynageo | geo | application/vnd.dynageo |
| 425 | apache-httpd/3616775988626861360 | application/vnd.ecowin.chart | mag | application/vnd.ecowin.chart |
| 426 | apache-httpd/9077911870478152775 | application/vnd.enliven | nml | application/vnd.enliven |
| 427 | apache-httpd/12706596273811258171 | application/vnd.epson.esf | esf | application/vnd.epson.esf |
| 428 | apache-httpd/14165040414098646264 | application/vnd.epson.msf | msf | application/vnd.epson.msf |
| 429 | apache-httpd/17134547263694623527 | application/vnd.epson.quickanime | qam | application/vnd.epson.quickanime |
| 430 | apache-httpd/16707635017572183154 | application/vnd.epson.salt | slt | application/vnd.epson.salt |
| 431 | apache-httpd/2712046030815474526 | application/vnd.epson.ssf | ssf | application/vnd.epson.ssf |
| 432 | apache-httpd/4209892087455402036 | application/vnd.eszigno3+xml | es3, et3 | application/vnd.eszigno3+xml |
| 433 | apache-httpd/9172159127052559283 | application/vnd.ezpix-album | ez2 | application/vnd.ezpix-album |
| 434 | apache-httpd/6305783307604018486 | application/vnd.ezpix-package | ez3 | application/vnd.ezpix-package |
| 435 | apache-httpd/4459766482740976956 | application/vnd.fdsn.mseed | mseed | application/vnd.fdsn.mseed |
| 436 | apache-httpd/8769941306517163787 | application/vnd.fdsn.seed | seed, dataless | application/vnd.fdsn.seed |
| 437 | apache-httpd/12678262227344080642 | application/vnd.flographit | gph | application/vnd.flographit |
| 438 | apache-httpd/4578033822823314924 | application/vnd.fluxtime.clip | ftc | application/vnd.fluxtime.clip |
| 439 | apache-httpd/12395607058870399443 | application/vnd.frogans.fnc | fnc | application/vnd.frogans.fnc |
| 440 | apache-httpd/8500864032604176996 | application/vnd.frogans.ltf | ltf | application/vnd.frogans.ltf |
| 441 | apache-httpd/17406533731142436187 | application/vnd.fsc.weblaunch | fsc | application/vnd.fsc.weblaunch |
| 442 | apache-httpd/12242543921585866057 | application/vnd.fujitsu.oasys | oas | application/vnd.fujitsu.oasys |
| 443 | apache-httpd/10445141258930429958 | application/vnd.fujitsu.oasys2 | oa2 | application/vnd.fujitsu.oasys2 |
| 444 | apache-httpd/10930448860814577903 | application/vnd.fujitsu.oasys3 | oa3 | application/vnd.fujitsu.oasys3 |
| 445 | apache-httpd/14387322337827212786 | application/vnd.fujitsu.oasysgp | fg5 | application/vnd.fujitsu.oasysgp |
| 446 | apache-httpd/16733117196381142994 | application/vnd.fujitsu.oasysprs | bh2 | application/vnd.fujitsu.oasysprs |
| 447 | apache-httpd/9711736421462602487 | application/vnd.fujixerox.ddd | ddd | application/vnd.fujixerox.ddd |
| 448 | apache-httpd/7397097853789512879 | application/vnd.fujixerox.docuworks | xdw | application/vnd.fujixerox.docuworks |
| 449 | apache-httpd/4615680608812704211 | application/vnd.fujixerox.docuworks.binder | xbd | application/vnd.fujixerox.docuworks.binder |
| 450 | apache-httpd/16352059154170337709 | application/vnd.fuzzysheet | fzs | application/vnd.fuzzysheet |
| 451 | apache-httpd/1498916370976491465 | application/vnd.genomatix.tuxedo | txd | application/vnd.genomatix.tuxedo |
| 452 | apache-httpd/12975698861315840269 | application/vnd.geogebra.slides | ggs | application/vnd.geogebra.slides |
| 453 | apache-httpd/5451843174200806045 | application/vnd.geogebra.tool | ggt | application/vnd.geogebra.tool |
| 454 | apache-httpd/17586183174821891669 | application/vnd.geometry-explorer | gex, gre | application/vnd.geometry-explorer |
| 455 | apache-httpd/10230197699255622710 | application/vnd.geonext | gxt | application/vnd.geonext |
| 456 | apache-httpd/18438421708258998726 | application/vnd.geoplan | g2w | application/vnd.geoplan |
| 457 | apache-httpd/15897574654348769304 | application/vnd.geospace | g3w | application/vnd.geospace |
| 458 | apache-httpd/18355248393964917169 | application/vnd.gmx | gmx | application/vnd.gmx |
| 459 | apache-httpd/15909748915402764967 | application/vnd.grafeq | gqf, gqs | application/vnd.grafeq |
| 460 | apache-httpd/12441779877712447627 | application/vnd.groove-account | gac | application/vnd.groove-account |
| 461 | apache-httpd/10596767586543562184 | application/vnd.groove-help | ghf | application/vnd.groove-help |
| 462 | apache-httpd/14392858622867149957 | application/vnd.groove-identity-message | gim | application/vnd.groove-identity-message |
| 463 | apache-httpd/3372644123074246335 | application/vnd.groove-injector | grv | application/vnd.groove-injector |
| 464 | apache-httpd/13266002556837886784 | application/vnd.groove-tool-message | gtm | application/vnd.groove-tool-message |
| 465 | apache-httpd/15180515305868542212 | application/vnd.groove-tool-template | tpl | application/vnd.groove-tool-template |
| 466 | apache-httpd/7294698077520185127 | application/vnd.groove-vcard | vcg | application/vnd.groove-vcard |
| 467 | apache-httpd/15717357483233793640 | application/vnd.hal+xml | hal | application/vnd.hal+xml |
| 468 | apache-httpd/3355380472191294597 | application/vnd.handheld-entertainment+xml | zmm | application/vnd.handheld-entertainment+xml |
| 469 | apache-httpd/921806164785967428 | application/vnd.hbci | hbci | application/vnd.hbci |
| 470 | apache-httpd/5614076178321165217 | application/vnd.hhe.lesson-player | les | application/vnd.hhe.lesson-player |
| 471 | apache-httpd/15216320534825696251 | application/vnd.hp-hpgl | hpgl | application/vnd.hp-hpgl |
| 472 | apache-httpd/2484029793804409931 | application/vnd.hp-hpid | hpid | application/vnd.hp-hpid |
| 473 | apache-httpd/10350728140267115011 | application/vnd.hp-hps | hps | application/vnd.hp-hps |
| 474 | apache-httpd/9662991493855678691 | application/vnd.hp-jlyt | jlt | application/vnd.hp-jlyt |
| 475 | apache-httpd/7002675830999037402 | application/vnd.hp-pcl | pcl | application/vnd.hp-pcl |
| 476 | apache-httpd/13986790209176469882 | application/vnd.hp-pclxl | pclxl | application/vnd.hp-pclxl |
| 477 | apache-httpd/3133176646220210526 | application/vnd.hydrostatix.sof-data | sfd-hdstx | application/vnd.hydrostatix.sof-data |
| 478 | apache-httpd/15991374402993791424 | application/vnd.ibm.minipay | mpy | application/vnd.ibm.minipay |
| 479 | apache-httpd/5227450111622812231 | application/vnd.ibm.modcap | afp, listafp, list3820 | application/vnd.ibm.modcap |
| 480 | apache-httpd/4568607398768371283 | application/vnd.ibm.rights-management | irm | application/vnd.ibm.rights-management |
| 481 | apache-httpd/2128091057912424249 | application/vnd.ibm.secure-container | sc | application/vnd.ibm.secure-container |
| 482 | apache-httpd/2229183100233788401 | application/vnd.igloader | igl | application/vnd.igloader |
| 483 | apache-httpd/17844121757852885447 | application/vnd.immervision-ivp | ivp | application/vnd.immervision-ivp |
| 484 | apache-httpd/15603360700258082080 | application/vnd.immervision-ivu | ivu | application/vnd.immervision-ivu |
| 485 | apache-httpd/12251698039906497039 | application/vnd.insors.igm | igm | application/vnd.insors.igm |
| 486 | apache-httpd/11618853048004134807 | application/vnd.intercon.formnet | xpw, xpx | application/vnd.intercon.formnet |
| 487 | apache-httpd/14360260915108678098 | application/vnd.intergeo | i2g | application/vnd.intergeo |
| 488 | apache-httpd/14725031458507571431 | application/vnd.intu.qbo | qbo | application/vnd.intu.qbo |
| 489 | apache-httpd/11066612415550178722 | application/vnd.intu.qfx | qfx | application/vnd.intu.qfx |
| 490 | apache-httpd/15706926158986829733 | application/vnd.ipunplugged.rcprofile | rcprofile | application/vnd.ipunplugged.rcprofile |
| 491 | apache-httpd/12093715921562340635 | application/vnd.irepository.package+xml | irp | application/vnd.irepository.package+xml |
| 492 | apache-httpd/1897709426287013613 | application/vnd.is-xpr | xpr | application/vnd.is-xpr |
| 493 | apache-httpd/13300239476686186637 | application/vnd.jam | jam | application/vnd.jam |
| 494 | apache-httpd/14660139996855751709 | application/vnd.jcp.javame.midlet-rms | rms | application/vnd.jcp.javame.midlet-rms |
| 495 | apache-httpd/18064817492816535970 | application/vnd.jisp | jisp | application/vnd.jisp |
| 496 | apache-httpd/15046818054618797090 | application/vnd.joost.joda-archive | joda | application/vnd.joost.joda-archive |
| 497 | apache-httpd/16430238005925662096 | application/vnd.kahootz | ktz, ktr | application/vnd.kahootz |
| 498 | apache-httpd/1900611674645073403 | application/vnd.kde.karbon | karbon | application/vnd.kde.karbon |
| 499 | apache-httpd/5868147151211495659 | application/vnd.kde.kchart | chrt | application/vnd.kde.kchart |
| 500 | apache-httpd/9081880592162353802 | application/vnd.kde.kformula | kfo | application/vnd.kde.kformula |
| 501 | apache-httpd/12077047024898395038 | application/vnd.kde.kivio | flw | application/vnd.kde.kivio |
| 502 | apache-httpd/3250382740988290933 | application/vnd.kde.kontour | kon | application/vnd.kde.kontour |
| 503 | apache-httpd/15363840241960418613 | application/vnd.kde.kpresenter | kpr, kpt | application/vnd.kde.kpresenter |
| 504 | apache-httpd/10622120689245270024 | application/vnd.kde.kspread | ksp | application/vnd.kde.kspread |
| 505 | apache-httpd/4053381207682315711 | application/vnd.kde.kword | kwd, kwt | application/vnd.kde.kword |
| 506 | apache-httpd/5556381308704270204 | application/vnd.kenameaapp | htke | application/vnd.kenameaapp |
| 507 | apache-httpd/6053998860560020025 | application/vnd.kidspiration | kia | application/vnd.kidspiration |
| 508 | apache-httpd/9056205338064453253 | application/vnd.kinar | kne, knp | application/vnd.kinar |
| 509 | apache-httpd/14857126558942962564 | application/vnd.koan | skp, skd, skt, skm | application/vnd.koan |
| 510 | apache-httpd/2264720326959224348 | application/vnd.kodak-descriptor | sse | application/vnd.kodak-descriptor |
| 511 | apache-httpd/1683673292716551156 | application/vnd.las.las+xml | lasxml | application/vnd.las.las+xml |
| 512 | apache-httpd/13042316040512874645 | application/vnd.llamagraphics.life-balance.desktop | lbd | application/vnd.llamagraphics.life-balance.desktop |
| 513 | apache-httpd/5029791183561743911 | application/vnd.llamagraphics.life-balance.exchange+xml | lbe | application/vnd.llamagraphics.life-balance.exchange+xml |
| 514 | apache-httpd/5593557214741015173 | application/vnd.lotus-freelance | pre | application/vnd.lotus-freelance |
| 515 | apache-httpd/12260496078258194652 | application/vnd.lotus-organizer | org | application/vnd.lotus-organizer |
| 516 | apache-httpd/5659107803445623294 | application/vnd.macports.portpkg | portpkg | application/vnd.macports.portpkg |
| 517 | apache-httpd/1187713301663608044 | application/vnd.mcd | mcd | application/vnd.mcd |
| 518 | apache-httpd/4072025194511321041 | application/vnd.medcalcdata | mc1 | application/vnd.medcalcdata |
| 519 | apache-httpd/5788543607611052916 | application/vnd.mediastation.cdkey | cdkey | application/vnd.mediastation.cdkey |
| 520 | apache-httpd/12340112869033537762 | application/vnd.mfer | mwf | application/vnd.mfer |
| 521 | apache-httpd/10582393778901387068 | application/vnd.mfmp | mfm | application/vnd.mfmp |
| 522 | apache-httpd/6898984041858146169 | application/vnd.micrografx.flo | flo | application/vnd.micrografx.flo |
| 523 | apache-httpd/15630303916909994538 | application/vnd.micrografx.igx | igx | application/vnd.micrografx.igx |
| 524 | apache-httpd/3942324793998983809 | application/vnd.mobius.daf | daf | application/vnd.mobius.daf |
| 525 | apache-httpd/12614044088270288356 | application/vnd.mobius.dis | dis | application/vnd.mobius.dis |
| 526 | apache-httpd/17302285957713971200 | application/vnd.mobius.mbk | mbk | application/vnd.mobius.mbk |
| 527 | apache-httpd/13588310119824942220 | application/vnd.mobius.mqy | mqy | application/vnd.mobius.mqy |
| 528 | apache-httpd/9094943142734310647 | application/vnd.mobius.msl | msl | application/vnd.mobius.msl |
| 529 | apache-httpd/4431472809167363624 | application/vnd.mobius.plc | plc | application/vnd.mobius.plc |
| 530 | apache-httpd/16639777093975277442 | application/vnd.mobius.txf | txf | application/vnd.mobius.txf |
| 531 | apache-httpd/17697489348673614307 | application/vnd.mophun.application | mpn | application/vnd.mophun.application |
| 532 | apache-httpd/5827300586678435774 | application/vnd.mophun.certificate | mpc | application/vnd.mophun.certificate |
| 533 | apache-httpd/10710046949140717356 | application/vnd.mozilla.xul+xml | xul | application/vnd.mozilla.xul+xml |
| 534 | apache-httpd/11231017852761258701 | application/vnd.ms-artgalry | cil | application/vnd.ms-artgalry |
| 535 | apache-httpd/7193524944753159053 | application/vnd.ms-excel.addin.macroenabled.12 | xlam | application/vnd.ms-excel.addin.macroenabled.12 |
| 536 | apache-httpd/17758141517366411449 | application/vnd.ms-excel.sheet.binary.macroenabled.12 | xlsb | application/vnd.ms-excel.sheet.binary.macroenabled.12 |
| 537 | apache-httpd/453850464475411149 | application/vnd.ms-excel.sheet.macroenabled.12 | xlsm | application/vnd.ms-excel.sheet.macroenabled.12 |
| 538 | apache-httpd/12161631093412850975 | application/vnd.ms-excel.template.macroenabled.12 | xltm | application/vnd.ms-excel.template.macroenabled.12 |
| 539 | apache-httpd/8850202862544404839 | application/vnd.ms-ims | ims | application/vnd.ms-ims |
| 540 | apache-httpd/9023937269523142240 | application/vnd.ms-lrm | lrm | application/vnd.ms-lrm |
| 541 | apache-httpd/5764838913370525935 | application/vnd.ms-pki.seccat | cat | application/vnd.ms-pki.seccat |
| 542 | apache-httpd/13175711861487568318 | application/vnd.ms-pki.stl | stl | application/vnd.ms-pki.stl |
| 543 | apache-httpd/17352955355019002013 | application/vnd.ms-powerpoint.addin.macroenabled.12 | ppam | application/vnd.ms-powerpoint.addin.macroenabled.12 |
| 544 | apache-httpd/2047390379536670552 | application/vnd.ms-powerpoint.presentation.macroenabled.12 | pptm | application/vnd.ms-powerpoint.presentation.macroenabled.12 |
| 545 | apache-httpd/8409127412828138689 | application/vnd.ms-powerpoint.slide.macroenabled.12 | sldm | application/vnd.ms-powerpoint.slide.macroenabled.12 |
| 546 | apache-httpd/6816715224970428638 | application/vnd.ms-powerpoint.slideshow.macroenabled.12 | ppsm | application/vnd.ms-powerpoint.slideshow.macroenabled.12 |
| 547 | apache-httpd/11696996066490782296 | application/vnd.ms-powerpoint.template.macroenabled.12 | potm | application/vnd.ms-powerpoint.template.macroenabled.12 |
| 548 | apache-httpd/6873454529924296618 | application/vnd.ms-word.document.macroenabled.12 | docm | application/vnd.ms-word.document.macroenabled.12 |
| 549 | apache-httpd/3685976441960773084 | application/vnd.ms-word.template.macroenabled.12 | dotm | application/vnd.ms-word.template.macroenabled.12 |
| 550 | apache-httpd/77905493598287429 | application/vnd.ms-works | wps, wks, wcm, wdb | application/vnd.ms-works |
| 551 | apache-httpd/13793382431961437112 | application/vnd.ms-xpsdocument | xps | application/vnd.ms-xpsdocument |
| 552 | apache-httpd/4781651560126321324 | application/vnd.mseq | mseq | application/vnd.mseq |
| 553 | apache-httpd/14549417581595720487 | application/vnd.musician | mus | application/vnd.musician |
| 554 | apache-httpd/6940775733493568946 | application/vnd.muvee.style | msty | application/vnd.muvee.style |
| 555 | apache-httpd/3675306534347999425 | application/vnd.mynfc | taglet | application/vnd.mynfc |
| 556 | apache-httpd/15808064807033990788 | application/vnd.neurolanguage.nlu | nlu | application/vnd.neurolanguage.nlu |
| 557 | apache-httpd/8715266759443811523 | application/vnd.noblenet-directory | nnd | application/vnd.noblenet-directory |
| 558 | apache-httpd/16428619282818802010 | application/vnd.noblenet-sealer | nns | application/vnd.noblenet-sealer |
| 559 | apache-httpd/16923941215688082693 | application/vnd.noblenet-web | nnw | application/vnd.noblenet-web |
| 560 | apache-httpd/14471098247504845837 | application/vnd.nokia.n-gage.data | ngdat | application/vnd.nokia.n-gage.data |
| 561 | apache-httpd/3421485305706899045 | application/vnd.nokia.n-gage.symbian.install | n-gage | application/vnd.nokia.n-gage.symbian.install |
| 562 | apache-httpd/14018862107975690802 | application/vnd.nokia.radio-preset | rpst | application/vnd.nokia.radio-preset |
| 563 | apache-httpd/5985527066767386131 | application/vnd.nokia.radio-presets | rpss | application/vnd.nokia.radio-presets |
| 564 | apache-httpd/1042982873375166231 | application/vnd.novadigm.edm | edm | application/vnd.novadigm.edm |
| 565 | apache-httpd/1300003046207835122 | application/vnd.novadigm.edx | edx | application/vnd.novadigm.edx |
| 566 | apache-httpd/12498182002620875355 | application/vnd.novadigm.ext | ext | application/vnd.novadigm.ext |
| 567 | apache-httpd/6962085545806922702 | application/vnd.oasis.opendocument.chart | odc | application/vnd.oasis.opendocument.chart |
| 568 | apache-httpd/9903176183460908302 | application/vnd.oasis.opendocument.chart-template | otc | application/vnd.oasis.opendocument.chart-template |
| 569 | apache-httpd/17995198403821469367 | application/vnd.oasis.opendocument.database | odb | application/vnd.oasis.opendocument.database |
| 570 | apache-httpd/4847972083146275462 | application/vnd.oasis.opendocument.formula | odf | application/vnd.oasis.opendocument.formula |
| 571 | apache-httpd/13471152099293494062 | application/vnd.oasis.opendocument.formula-template | odft | application/vnd.oasis.opendocument.formula-template |
| 572 | apache-httpd/7221080215284733600 | application/vnd.oasis.opendocument.graphics-template | otg | application/vnd.oasis.opendocument.graphics-template |
| 573 | apache-httpd/7212275662721677375 | application/vnd.oasis.opendocument.image | odi | application/vnd.oasis.opendocument.image |
| 574 | apache-httpd/7696442023566449237 | application/vnd.oasis.opendocument.image-template | oti | application/vnd.oasis.opendocument.image-template |
| 575 | apache-httpd/3214923385420216598 | application/vnd.oasis.opendocument.presentation-template | otp | application/vnd.oasis.opendocument.presentation-template |
| 576 | apache-httpd/13058603825924308571 | application/vnd.oasis.opendocument.spreadsheet-template | ots | application/vnd.oasis.opendocument.spreadsheet-template |
| 577 | apache-httpd/16625459860859088264 | application/vnd.oasis.opendocument.text-master | odm | application/vnd.oasis.opendocument.text-master |
| 578 | apache-httpd/11169527702873022564 | application/vnd.oasis.opendocument.text-template | ott | application/vnd.oasis.opendocument.text-template |
| 579 | apache-httpd/14133275647000193392 | application/vnd.oasis.opendocument.text-web | oth | application/vnd.oasis.opendocument.text-web |
| 580 | apache-httpd/13950687050277105287 | application/vnd.olpc-sugar | xo | application/vnd.olpc-sugar |
| 581 | apache-httpd/3732919740460660496 | application/vnd.oma.dd2+xml | dd2 | application/vnd.oma.dd2+xml |
| 582 | apache-httpd/9058684993375571790 | application/vnd.openofficeorg.extension | oxt | application/vnd.openofficeorg.extension |
| 583 | apache-httpd/535144868066071015 | application/vnd.openxmlformats-officedocument.presentationml.slide | sldx | application/vnd.openxmlformats-officedocument.presentationml.slide |
| 584 | apache-httpd/5150666565402240513 | application/vnd.osgeo.mapguide.package | mgp | application/vnd.osgeo.mapguide.package |
| 585 | apache-httpd/9288499656011032268 | application/vnd.osgi.dp | dp | application/vnd.osgi.dp |
| 586 | apache-httpd/4956746657128679058 | application/vnd.osgi.subsystem | esa | application/vnd.osgi.subsystem |
| 587 | apache-httpd/8177121348548023314 | application/vnd.palm | pdb, pqa, oprc | application/vnd.palm |
| 588 | apache-httpd/5060715562509657656 | application/vnd.pawaafile | paw | application/vnd.pawaafile |
| 589 | apache-httpd/3445346377631037340 | application/vnd.pg.format | str | application/vnd.pg.format |
| 590 | apache-httpd/11105274837093552377 | application/vnd.pg.osasli | ei6 | application/vnd.pg.osasli |
| 591 | apache-httpd/18162782278582436098 | application/vnd.picsel | efif | application/vnd.picsel |
| 592 | apache-httpd/2471663342369167330 | application/vnd.pmi.widget | wg | application/vnd.pmi.widget |
| 593 | apache-httpd/17072281593186703027 | application/vnd.pocketlearn | plf | application/vnd.pocketlearn |
| 594 | apache-httpd/13637343208873454390 | application/vnd.powerbuilder6 | pbd | application/vnd.powerbuilder6 |
| 595 | apache-httpd/16902732272208064392 | application/vnd.previewsystems.box | box | application/vnd.previewsystems.box |
| 596 | apache-httpd/17614153492566206582 | application/vnd.proteus.magazine | mgz | application/vnd.proteus.magazine |
| 597 | apache-httpd/7428950758947074420 | application/vnd.publishare-delta-tree | qps | application/vnd.publishare-delta-tree |
| 598 | apache-httpd/18177019580753205670 | application/vnd.pvi.ptid1 | ptid | application/vnd.pvi.ptid1 |
| 599 | apache-httpd/18202258566732886046 | application/vnd.quark.quarkxpress | qxd, qxt, qwd, qwt, qxl, qxb | application/vnd.quark.quarkxpress |
| 600 | apache-httpd/9040307230090136548 | application/vnd.realvnc.bed | bed | application/vnd.realvnc.bed |
| 601 | apache-httpd/8134742735002335236 | application/vnd.rig.cryptonote | cryptonote | application/vnd.rig.cryptonote |
| 602 | apache-httpd/17335134041164572274 | application/vnd.rim.cod | cod | application/vnd.rim.cod |
| 603 | apache-httpd/18248750328496013465 | application/vnd.rn-realmedia-vbr | rmvb | application/vnd.rn-realmedia-vbr |
| 604 | apache-httpd/15782473511556218270 | application/vnd.route66.link66+xml | link66 | application/vnd.route66.link66+xml |
| 605 | apache-httpd/6902652129543922508 | application/vnd.sailingtracker.track | st | application/vnd.sailingtracker.track |
| 606 | apache-httpd/3558484330179102172 | application/vnd.seemail | see | application/vnd.seemail |
| 607 | apache-httpd/9088264634842123326 | application/vnd.sema | sema | application/vnd.sema |
| 608 | apache-httpd/7766891016013321425 | application/vnd.semd | semd | application/vnd.semd |
| 609 | apache-httpd/605004890228926996 | application/vnd.semf | semf | application/vnd.semf |
| 610 | apache-httpd/10173667125709949723 | application/vnd.shana.informed.formdata | ifm | application/vnd.shana.informed.formdata |
| 611 | apache-httpd/6961800999383898337 | application/vnd.shana.informed.formtemplate | itp | application/vnd.shana.informed.formtemplate |
| 612 | apache-httpd/14364337678005556784 | application/vnd.shana.informed.interchange | iif | application/vnd.shana.informed.interchange |
| 613 | apache-httpd/10655404354733431138 | application/vnd.shana.informed.package | ipk | application/vnd.shana.informed.package |
| 614 | apache-httpd/10132205750207181206 | application/vnd.simtech-mindmapper | twd, twds | application/vnd.simtech-mindmapper |
| 615 | apache-httpd/5348486227401194865 | application/vnd.smaf | mmf | application/vnd.smaf |
| 616 | apache-httpd/17112233685912069940 | application/vnd.smart.teacher | teacher | application/vnd.smart.teacher |
| 617 | apache-httpd/14282171209386449834 | application/vnd.solent.sdkm+xml | sdkm, sdkd | application/vnd.solent.sdkm+xml |
| 618 | apache-httpd/15134871687334348672 | application/vnd.spotfire.dxp | dxp | application/vnd.spotfire.dxp |
| 619 | apache-httpd/9182902311269733745 | application/vnd.spotfire.sfs | sfs | application/vnd.spotfire.sfs |
| 620 | apache-httpd/13156498984566289220 | application/vnd.stardivision.calc | sdc | application/vnd.stardivision.calc |
| 621 | apache-httpd/17575372223664686893 | application/vnd.stardivision.impress | sdd | application/vnd.stardivision.impress |
| 622 | apache-httpd/17689098337773368263 | application/vnd.stardivision.math | smf | application/vnd.stardivision.math |
| 623 | apache-httpd/2615400360932218329 | application/vnd.stardivision.writer-global | sgl | application/vnd.stardivision.writer-global |
| 624 | apache-httpd/6070408414635237449 | application/vnd.stepmania.package | smzip | application/vnd.stepmania.package |
| 625 | apache-httpd/12775287888660207002 | application/vnd.stepmania.stepchart | sm | application/vnd.stepmania.stepchart |
| 626 | apache-httpd/17986501611929889311 | application/vnd.sun.xml.calc.template | stc | application/vnd.sun.xml.calc.template |
| 627 | apache-httpd/6782189470640064892 | application/vnd.sun.xml.draw.template | std | application/vnd.sun.xml.draw.template |
| 628 | apache-httpd/9051819284956207821 | application/vnd.sun.xml.impress.template | sti | application/vnd.sun.xml.impress.template |
| 629 | apache-httpd/13582896711578522359 | application/vnd.sun.xml.math | sxm | application/vnd.sun.xml.math |
| 630 | apache-httpd/13923193796103206743 | application/vnd.sun.xml.writer.global | sxg | application/vnd.sun.xml.writer.global |
| 631 | apache-httpd/8267445119885387718 | application/vnd.sun.xml.writer.template | stw | application/vnd.sun.xml.writer.template |
| 632 | apache-httpd/10548945477827165241 | application/vnd.sus-calendar | sus, susp | application/vnd.sus-calendar |
| 633 | apache-httpd/7128109369329973531 | application/vnd.svd | svd | application/vnd.svd |
| 634 | apache-httpd/16923033345032457 | application/vnd.symbian.install | sis, sisx | application/vnd.symbian.install |
| 635 | apache-httpd/16688631424076391908 | application/vnd.syncml+xml | xsm | application/vnd.syncml+xml |
| 636 | apache-httpd/11342459016954218595 | application/vnd.syncml.dm+wbxml | bdm | application/vnd.syncml.dm+wbxml |
| 637 | apache-httpd/18350820160319142221 | application/vnd.syncml.dm+xml | xdm | application/vnd.syncml.dm+xml |
| 638 | apache-httpd/9814302341999196290 | application/vnd.tao.intent-module-archive | tao | application/vnd.tao.intent-module-archive |
| 639 | apache-httpd/946043484823141794 | application/vnd.tmobile-livetv | tmo | application/vnd.tmobile-livetv |
| 640 | apache-httpd/7943339296956635151 | application/vnd.trid.tpt | tpt | application/vnd.trid.tpt |
| 641 | apache-httpd/11556186021095517549 | application/vnd.triscape.mxs | mxs | application/vnd.triscape.mxs |
| 642 | apache-httpd/918813325830249825 | application/vnd.trueapp | tra | application/vnd.trueapp |
| 643 | apache-httpd/18233141180189460471 | application/vnd.ufdl | ufd, ufdl | application/vnd.ufdl |
| 644 | apache-httpd/13562036474254073458 | application/vnd.uiq.theme | utz | application/vnd.uiq.theme |
| 645 | apache-httpd/5595356910571652011 | application/vnd.umajin | umj | application/vnd.umajin |
| 646 | apache-httpd/18397815986228800869 | application/vnd.unity | unityweb | application/vnd.unity |
| 647 | apache-httpd/8338995056510031503 | application/vnd.uoml+xml | uoml | application/vnd.uoml+xml |
| 648 | apache-httpd/16181652763127625250 | application/vnd.vcx | vcx | application/vnd.vcx |
| 649 | apache-httpd/4738532386085573748 | application/vnd.visionary | vis | application/vnd.visionary |
| 650 | apache-httpd/17932304242732550516 | application/vnd.vsf | vsf | application/vnd.vsf |
| 651 | apache-httpd/866180193996727481 | application/vnd.wap.wbxml | wbxml | application/vnd.wap.wbxml |
| 652 | apache-httpd/13947353468228148623 | application/vnd.wap.wmlc | wmlc | application/vnd.wap.wmlc |
| 653 | apache-httpd/15817187641996057495 | application/vnd.wap.wmlscriptc | wmlsc | application/vnd.wap.wmlscriptc |
| 654 | apache-httpd/11891847532744855460 | application/vnd.webturbo | wtb | application/vnd.webturbo |
| 655 | apache-httpd/10008482833353115188 | application/vnd.wolfram.player | nbp | application/vnd.wolfram.player |
| 656 | apache-httpd/17504618014497501474 | application/vnd.wqd | wqd | application/vnd.wqd |
| 657 | apache-httpd/1680085651608889131 | application/vnd.wt.stf | stf | application/vnd.wt.stf |
| 658 | apache-httpd/9915205638149722777 | application/vnd.xara | xar | application/vnd.xara |
| 659 | apache-httpd/16561078434078613689 | application/vnd.xfdl | xfdl | application/vnd.xfdl |
| 660 | apache-httpd/10771167679637303936 | application/vnd.yamaha.hv-dic | hvd | application/vnd.yamaha.hv-dic |
| 661 | apache-httpd/3755495095313444404 | application/vnd.yamaha.hv-script | hvs | application/vnd.yamaha.hv-script |
| 662 | apache-httpd/15459861044985438947 | application/vnd.yamaha.hv-voice | hvp | application/vnd.yamaha.hv-voice |
| 663 | apache-httpd/10227406980888024354 | application/vnd.yamaha.openscoreformat | osf | application/vnd.yamaha.openscoreformat |
| 664 | apache-httpd/10480217407537137783 | application/vnd.yamaha.openscoreformat.osfpvg+xml | osfpvg | application/vnd.yamaha.openscoreformat.osfpvg+xml |
| 665 | apache-httpd/11137728069844962273 | application/vnd.yamaha.smaf-audio | saf | application/vnd.yamaha.smaf-audio |
| 666 | apache-httpd/2616789489216992815 | application/vnd.yamaha.smaf-phrase | spf | application/vnd.yamaha.smaf-phrase |
| 667 | apache-httpd/18354551720029817871 | application/vnd.yellowriver-custom-menu | cmp | application/vnd.yellowriver-custom-menu |
| 668 | apache-httpd/15124211333922682931 | application/vnd.zul | zir, zirz | application/vnd.zul |
| 669 | apache-httpd/12435334305085697032 | application/vnd.zzazz.deck+xml | zaz | application/vnd.zzazz.deck+xml |
| 670 | apache-httpd/7319119812613679345 | application/voicexml+xml | vxml | application/voicexml+xml |
| 671 | apache-httpd/4399502372078247101 | application/wasm | wasm | application/wasm |
| 672 | apache-httpd/14032609428049303413 | application/widget | wgt | application/widget |
| 673 | apache-httpd/13710016921932272208 | application/winhlp | hlp | application/winhlp |
| 674 | apache-httpd/2487306939359366350 | application/wsdl+xml | wsdl | application/wsdl+xml |
| 675 | apache-httpd/269849346801687012 | application/wspolicy+xml | wspolicy | application/wspolicy+xml |
| 676 | apache-httpd/12448632667719045031 | application/x-7z-compressed | 7z | application/x-7z-compressed |
| 677 | apache-httpd/13077937943514110784 | application/x-abiword | abw | application/x-abiword |
| 678 | apache-httpd/9946846471999063966 | application/x-ace-compressed | ace | application/x-ace-compressed |
| 679 | apache-httpd/6628203580258632388 | application/x-authorware-bin | aab, x32, u32, vox | application/x-authorware-bin |
| 680 | apache-httpd/6257513896690260698 | application/x-authorware-map | aam | application/x-authorware-map |
| 681 | apache-httpd/7946561646438379956 | application/x-authorware-seg | aas | application/x-authorware-seg |
| 682 | apache-httpd/17607384768001508299 | application/x-bcpio | bcpio | application/x-bcpio |
| 683 | apache-httpd/8096259863664211066 | application/x-bittorrent | torrent | application/x-bittorrent |
| 684 | apache-httpd/13372282205074757825 | application/x-blorb | blb, blorb | application/x-blorb |
| 685 | apache-httpd/18270236438859893354 | application/x-bzip | bz | application/x-bzip |
| 686 | apache-httpd/6866278513552239343 | application/x-cbr | cbr, cba, cbt, cbz, cb7 | application/x-cbr |
| 687 | apache-httpd/3212196054821300571 | application/x-cdlink | vcd | application/x-cdlink |
| 688 | apache-httpd/11202271241335613371 | application/x-cfs-compressed | cfs | application/x-cfs-compressed |
| 689 | apache-httpd/3262656446900862880 | application/x-chat | chat | application/x-chat |
| 690 | apache-httpd/14847441681126817338 | application/x-chess-pgn | pgn | application/x-chess-pgn |
| 691 | apache-httpd/4068569377388053849 | application/x-conference | nsc | application/x-conference |
| 692 | apache-httpd/12237965542013894657 | application/x-cpio | cpio | application/x-cpio |
| 693 | apache-httpd/4465590080004403544 | application/x-csh | csh | application/x-csh |
| 694 | apache-httpd/1860797218639255065 | application/x-debian-package | deb, udeb | application/x-debian-package |
| 695 | apache-httpd/12491493047811765531 | application/x-dgc-compressed | dgc | application/x-dgc-compressed |
| 696 | apache-httpd/6150027229732099657 | application/x-doom | wad | application/x-doom |
| 697 | apache-httpd/3969505105703786494 | application/x-dtbncx+xml | ncx | application/x-dtbncx+xml |
| 698 | apache-httpd/12913620085324016297 | application/x-dtbook+xml | dtb | application/x-dtbook+xml |
| 699 | apache-httpd/3686352884682951143 | application/x-dtbresource+xml | res | application/x-dtbresource+xml |
| 700 | apache-httpd/11927020025323593929 | application/x-envoy | evy | application/x-envoy |
| 701 | apache-httpd/3083675910552425676 | application/x-eva | eva | application/x-eva |
| 702 | apache-httpd/17640913485785347964 | application/x-font-bdf | bdf | application/x-font-bdf |
| 703 | apache-httpd/17699946521673092095 | application/x-font-ghostscript | gsf | application/x-font-ghostscript |
| 704 | apache-httpd/16860774818483330147 | application/x-font-linux-psf | psf | application/x-font-linux-psf |
| 705 | apache-httpd/11747660559347456894 | application/x-font-pcf | pcf | application/x-font-pcf |
| 706 | apache-httpd/11006743696098534664 | application/x-font-snf | snf | application/x-font-snf |
| 707 | apache-httpd/9787351572380181968 | application/x-font-type1 | pfa, pfb, pfm, afm | application/x-font-type1 |
| 708 | apache-httpd/2127115815514234195 | application/x-freearc | arc | application/x-freearc |
| 709 | apache-httpd/5401117936824263590 | application/x-futuresplash | spl | application/x-futuresplash |
| 710 | apache-httpd/16906692640029232947 | application/x-gca-compressed | gca | application/x-gca-compressed |
| 711 | apache-httpd/11770528808789065522 | application/x-glulx | ulx | application/x-glulx |
| 712 | apache-httpd/17061360333974647667 | application/x-gramps-xml | gramps | application/x-gramps-xml |
| 713 | apache-httpd/9798185628672492787 | application/x-gtar | gtar | application/x-gtar |
| 714 | apache-httpd/16035530569936599317 | application/x-hdf | hdf | application/x-hdf |
| 715 | apache-httpd/15469217397881902467 | application/x-install-instructions | install | application/x-install-instructions |
| 716 | apache-httpd/9945333620512882087 | application/x-iso9660-image | iso | application/x-iso9660-image |
| 717 | apache-httpd/2333932034322286286 | application/x-java-jnlp-file | jnlp | application/x-java-jnlp-file |
| 718 | apache-httpd/11028599860578863387 | application/x-latex | latex | application/x-latex |
| 719 | apache-httpd/487741630637929822 | application/x-lzh-compressed | lzh, lha | application/x-lzh-compressed |
| 720 | apache-httpd/11881934563155833868 | application/x-mie | mie | application/x-mie |
| 721 | apache-httpd/17681871278090163123 | application/x-mobipocket-ebook | prc, mobi | application/x-mobipocket-ebook |
| 722 | apache-httpd/16627114407478665957 | application/x-ms-application | application | application/x-ms-application |
| 723 | apache-httpd/17484877487305115669 | application/x-ms-shortcut | lnk | application/x-ms-shortcut |
| 724 | apache-httpd/12139814779460625487 | application/x-ms-wmd | wmd | application/x-ms-wmd |
| 725 | apache-httpd/6444568146336275519 | application/x-ms-wmz | wmz | application/x-ms-wmz |
| 726 | apache-httpd/14888301265473117000 | application/x-ms-xbap | xbap | application/x-ms-xbap |
| 727 | apache-httpd/5216488401283368545 | application/x-msaccess | mdb | application/x-msaccess |
| 728 | apache-httpd/12151911956262664389 | application/x-msbinder | obd | application/x-msbinder |
| 729 | apache-httpd/5622317916550356859 | application/x-mscardfile | crd | application/x-mscardfile |
| 730 | apache-httpd/7535922962243314868 | application/x-msclip | clp | application/x-msclip |
| 731 | apache-httpd/9712560292327597037 | application/x-msdownload | exe, dll, com, bat, msi | application/x-msdownload |
| 732 | apache-httpd/10209449085056580529 | application/x-msmediaview | mvb, m13, m14 | application/x-msmediaview |
| 733 | apache-httpd/4486161733139916318 | application/x-msmetafile | wmf, wmz, emf, emz | application/x-msmetafile |
| 734 | apache-httpd/940430676671419282 | application/x-msmoney | mny | application/x-msmoney |
| 735 | apache-httpd/11706933179567706948 | application/x-msschedule | scd | application/x-msschedule |
| 736 | apache-httpd/17488117191040683567 | application/x-msterminal | trm | application/x-msterminal |
| 737 | apache-httpd/368739558323144978 | application/x-mswrite | wri | application/x-mswrite |
| 738 | apache-httpd/15382676714485689448 | application/x-nzb | nzb | application/x-nzb |
| 739 | apache-httpd/17536636616578527298 | application/x-pkcs12 | p12, pfx | application/x-pkcs12 |
| 740 | apache-httpd/10927828397113750156 | application/x-pkcs7-certificates | p7b, spc | application/x-pkcs7-certificates |
| 741 | apache-httpd/12396323594280000532 | application/x-pkcs7-certreqresp | p7r | application/x-pkcs7-certreqresp |
| 742 | apache-httpd/16410212459293594184 | application/x-rar-compressed | rar | application/x-rar-compressed |
| 743 | apache-httpd/15414754435807971579 | application/x-sh | sh | application/x-sh |
| 744 | apache-httpd/6212653248367935630 | application/x-silverlight-app | xap | application/x-silverlight-app |
| 745 | apache-httpd/5696044814271161966 | application/x-sql | sql | application/x-sql |
| 746 | apache-httpd/1762984859953048466 | application/x-stuffitx | sitx | application/x-stuffitx |
| 747 | apache-httpd/12175115353246933006 | application/x-subrip | srt | application/x-subrip |
| 748 | apache-httpd/7226557976762131676 | application/x-sv4cpio | sv4cpio | application/x-sv4cpio |
| 749 | apache-httpd/17467597544118241061 | application/x-sv4crc | sv4crc | application/x-sv4crc |
| 750 | apache-httpd/15083548841589849073 | application/x-t3vm-image | t3 | application/x-t3vm-image |
| 751 | apache-httpd/2705387182677033387 | application/x-tads | gam | application/x-tads |
| 752 | apache-httpd/15812306591423857328 | application/x-tcl | tcl | application/x-tcl |
| 753 | apache-httpd/3876443208898332249 | application/x-tex | tex | application/x-tex |
| 754 | apache-httpd/17620094873161353617 | application/x-tex-tfm | tfm | application/x-tex-tfm |
| 755 | apache-httpd/11775578050658360080 | application/x-texinfo | texinfo, texi | application/x-texinfo |
| 756 | apache-httpd/4451768154165309081 | application/x-tgif | obj | application/x-tgif |
| 757 | apache-httpd/347415101871463480 | application/x-ustar | ustar | application/x-ustar |
| 758 | apache-httpd/12506654617699971632 | application/x-wais-source | src | application/x-wais-source |
| 759 | apache-httpd/16814499624578705614 | application/x-x509-ca-cert | der, crt | application/x-x509-ca-cert |
| 760 | apache-httpd/10075710292268051668 | application/x-xfig | fig | application/x-xfig |
| 761 | apache-httpd/3716723091890390689 | application/x-xliff+xml | xlf | application/x-xliff+xml |
| 762 | apache-httpd/4000171426298219438 | application/x-xpinstall | xpi | application/x-xpinstall |
| 763 | apache-httpd/4433712085291521474 | application/x-xz | xz | application/x-xz |
| 764 | apache-httpd/17719390468068402583 | application/x-zmachine | z1, z2, z3, z4, z5, z6, z7, z8 | application/x-zmachine |
| 765 | apache-httpd/12986059742410892224 | application/xaml+xml | xaml | application/xaml+xml |
| 766 | apache-httpd/7452515137701384940 | application/xcap-diff+xml | xdf | application/xcap-diff+xml |
| 767 | apache-httpd/5638618832123976135 | application/xenc+xml | xenc | application/xenc+xml |
| 768 | apache-httpd/17748263794182868405 | application/xhtml+xml | xhtml, xht | application/xhtml+xml |
| 769 | apache-httpd/3113384014745721199 | application/xml-dtd | dtd | application/xml-dtd |
| 770 | apache-httpd/10149397603949644781 | application/xop+xml | xop | application/xop+xml |
| 771 | apache-httpd/16421235721521251338 | application/xproc+xml | xpl | application/xproc+xml |
| 772 | apache-httpd/6440939773538770935 | application/xslt+xml | xslt | application/xslt+xml |
| 773 | apache-httpd/11225691474892377083 | application/xspf+xml | xspf | application/xspf+xml |
| 774 | apache-httpd/1427004451618818651 | application/xv+xml | mxml, xhvml, xvml, xvm | application/xv+xml |
| 775 | apache-httpd/12313980582379923598 | application/yang | yang | application/yang |
| 776 | apache-httpd/8558864974377691069 | application/yin+xml | yin | application/yin+xml |
| 777 | x-fmt/228 | Applixware Bitmap | im |  |
| 778 | x-fmt/220 | Applixware Spreadsheet | as |  |
| 779 | linguist/20 | Arc | .arc |  |
| 780 | fmt/1473 | Archimedes Tracker Module | musx |  |
| 781 | fmt/1835 | Archiver Format | a |  |
| 782 | fmt/1833 | ArcSoft Album and SlideShow Files for PhotoStudio and PhotoImpression | abm, sld |  |
| 783 | fmt/1832 | ArcSoft PhotoStudio File | psf |  |
| 784 | fmt/610 | ARJ File Format | arj |  |
| 785 | fmt/666 | ART image format | art |  |
| 786 | fmt/1623 | Art Of Noise | aon |  |
| 787 | fmt/1624 | Art Of Noise | aon |  |
| 788 | fmt/1660 | Arts & Letters Clip Art Library | yal |  |
| 789 | fmt/1458 | Arts & Letters Graphics File | ged |  |
| 790 | linguist/22 | AsciiDoc | .asciidoc, .adoc, .asc |  |
| 791 | fmt/1592 | ASEG-GDF2 Description File | des |  |
| 792 | fmt/1593 | ASEG-GDF2- Data Definition File | dfn |  |
| 793 | linguist/124996147 | ASL | .asl, .dsl |  |
| 794 | linguist/7 | ASN.1 | .asn, .asn1 | text/x-ttcn-asn |
| 795 | fmt/1080 | ASP Application Directive File | asax |  |
| 796 | fmt/1081 | ASP Control Directive File | ascx |  |
| 797 | fmt/1082 | ASP WebService Directive File | asmx |  |
| 798 | linguist/564186416 | ASP.NET | .asax, .ascx, .ashx, .asmx, .aspx, .axd | application/x-aspx |
| 799 | linguist/23 | AspectJ | .aj |  |
| 800 | fmt/368 | ASPRS Lidar Data Exchange Format | las, laz |  |
| 801 | fmt/370 | ASPRS Lidar Data Exchange Format | las, laz |  |
| 802 | fmt/369 | ASPRS Lidar Data Exchange Format | las, laz |  |
| 803 | linguist/24 | Assembly | .asm, .a51, .i, .inc, .nas, .nasm |  |
| 804 | fmt/1564 | Associated Signature Container Extended (ASiC-E) | asice, sce | application/vnd.etsi.asic-e+zip |
| 805 | fmt/1341 | Associated Signature Container Simple (ASiC-S) | asics, scs | application/vnd.etsi.asic-s+zip |
| 806 | fmt/643 | ASTM E57 3D File Format | e57 | model/e57 |
| 807 | linguist/578209015 | Astro | .astro | text/jsx |
| 808 | fmt/1622 | Asylum Music Format | amf |  |
| 809 | fmt/1693 | Asymetrix Compel Presentation | cpl, art |  |
| 810 | fmt/1694 | Asymetrix Compel Presentation | cpl, art |  |
| 811 | fmt/470 | Asymetrix Toolbook File | tbk, sbk |  |
| 812 | fmt/1795 | Asymetrix Toolbook File | tbk, sbk |  |
| 813 | linguist/591605007 | Asymptote | .asy | text/x-kotlin |
| 814 | fmt/495 | ATCO-CIF | cif |  |
| 815 | fmt/1968 | Atrac Codec File | aea |  |
| 816 | linguist/9 | ATS | .dats, .hats, .sats |  |
| 817 | fmt/1822 | Audacity Audio Block File | au |  |
| 818 | fmt/1824 | Audacity Project File | aup |  |
| 819 | fmt/1826 | Audacity Project File | aup3 |  |
| 820 | fmt/1825 | Audacity Project File | aup |  |
| 821 | fmt/1823 | Audacity Project File | aup |  |
| 822 | fmt/1812 | Audio Data Transport Stream | aac, adts | audio/aac, audio/vnd.dlna.adts |
| 823 | x-fmt/135 | Audio Interchange File Format |  |  |
| 824 | fmt/414 | Audio Interchange File Format | aif, aiff |  |
| 825 | x-fmt/136 | Audio Interchange File Format (compressed) | aifc | audio/x-aiff |
| 826 | apache-httpd/2263091937087112692 | audio/adpcm | adp | audio/adpcm |
| 827 | apache-httpd/13380737628036455431 | audio/mp4 | m4a, mp4a | audio/mp4 |
| 828 | apache-httpd/14460022341763891094 | audio/s3m | s3m | audio/s3m |
| 829 | apache-httpd/17376429027797396733 | audio/silk | sil | audio/silk |
| 830 | fmt/5 | Audio/Video Interleaved Format | avi | video/x-msvideo |
| 831 | apache-httpd/9632120318872693343 | audio/vnd.dece.audio | uva, uvva | audio/vnd.dece.audio |
| 832 | apache-httpd/2543299767682612645 | audio/vnd.digital-winds | eol | audio/vnd.digital-winds |
| 833 | apache-httpd/10646039219402533391 | audio/vnd.dra | dra | audio/vnd.dra |
| 834 | apache-httpd/5184721221333372627 | audio/vnd.dts.hd | dtshd | audio/vnd.dts.hd |
| 835 | apache-httpd/13708406258791595207 | audio/vnd.lucent.voice | lvp | audio/vnd.lucent.voice |
| 836 | apache-httpd/11414351502892999160 | audio/vnd.ms-playready.media.pya | pya | audio/vnd.ms-playready.media.pya |
| 837 | apache-httpd/13130956661305356843 | audio/vnd.nuera.ecelp4800 | ecelp4800 | audio/vnd.nuera.ecelp4800 |
| 838 | apache-httpd/11322486937063478154 | audio/vnd.nuera.ecelp7470 | ecelp7470 | audio/vnd.nuera.ecelp7470 |
| 839 | apache-httpd/7473810765811430167 | audio/vnd.nuera.ecelp9600 | ecelp9600 | audio/vnd.nuera.ecelp9600 |
| 840 | apache-httpd/3181405020922865967 | audio/vnd.rip | rip | audio/vnd.rip |
| 841 | apache-httpd/8348285458340242731 | audio/webm | weba | audio/webm |
| 842 | apache-httpd/8181556888019162819 | audio/x-aac | aac | audio/x-aac |
| 843 | apache-httpd/8411735535461836205 | audio/x-caf | caf | audio/x-caf |
| 844 | apache-httpd/12318121939912865041 | audio/x-flac | flac | audio/x-flac |
| 845 | apache-httpd/5069105610641688063 | audio/x-matroska | mka | audio/x-matroska |
| 846 | apache-httpd/11650175986876150549 | audio/x-mpegurl | m3u | audio/x-mpegurl |
| 847 | apache-httpd/16874531211968188565 | audio/x-ms-wax | wax | audio/x-ms-wax |
| 848 | apache-httpd/17710107248733465858 | audio/x-pn-realaudio-plugin | rmp | audio/x-pn-realaudio-plugin |
| 849 | linguist/25 | Augeas | .aug |  |
| 850 | fmt/1939 | Auto FX PhotoGraphic Edges Image File | afx |  |
| 851 | x-fmt/98 | AutoCAD ACIS Export File | sat |  |
| 852 | x-fmt/26 | AutoCAD Batch Plot File | bp2, bpl |  |
| 853 | x-fmt/27 | AutoCAD Batch Plot File | bp3 |  |
| 854 | x-fmt/24 | AutoCAD Block Attribute Template | blk |  |
| 855 | x-fmt/37 | AutoCAD Colour-Dependant Plot Style Table | ctb |  |
| 856 | x-fmt/68 | AutoCAD Compiled Menu | mnc |  |
| 857 | x-fmt/103 | AutoCAD Compiled Shape/Font File | shx |  |
| 858 | x-fmt/38 | AutoCAD Custom Dictionary | cus |  |
| 859 | x-fmt/441 | AutoCAD Database File Locking Information | dwl | application/octet-stream |
| 860 | x-fmt/39 | AutoCAD dbConnect Query Set | dbq |  |
| 861 | x-fmt/40 | AutoCAD dbConnect Template Set | dbt |  |
| 862 | x-fmt/49 | AutoCAD Design Web Format | dwf | application/dwf, application/x-dwf, drawing/x-dwf, image/vnd.dwf, image/x-dwf, model/vnd.dwf |
| 863 | fmt/977 | AutoCAD Design Web Format(DWFx) | dwfx |  |
| 864 | x-fmt/134 | AutoCAD Device-Independent Binary Plotter File | adi |  |
| 865 | fmt/1456 | Autocad DMP File | dmp |  |
| 866 | fmt/32 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 867 | fmt/22 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 868 | fmt/31 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 869 | fmt/21 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 870 | fmt/29 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 871 | fmt/34 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 872 | fmt/23 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 873 | fmt/27 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 874 | fmt/33 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 875 | fmt/434 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 876 | fmt/36 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 877 | fmt/26 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 878 | fmt/1395 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 879 | fmt/25 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 880 | fmt/24 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 881 | fmt/30 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 882 | fmt/28 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 883 | fmt/531 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 884 | x-fmt/455 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 885 | fmt/35 | AutoCAD Drawing | dwg | image/vnd.dwg |
| 886 | x-fmt/50 | AutoCAD Drawing Standards File | dws |  |
| 887 | x-fmt/51 | AutoCAD Drawing Template | dwt |  |
| 888 | x-fmt/112 | AutoCAD External Database Configuration File | udl |  |
| 889 | x-fmt/155 | AutoCAD Film Roll | flm |  |
| 890 | x-fmt/54 | AutoCAD Font Mapping Table | fmp |  |
| 891 | fmt/1103 | AutoCAD Hatch Pattern | pat |  |
| 892 | x-fmt/61 | AutoCAD Landscape Library | lli |  |
| 893 | x-fmt/59 | AutoCAD Last Saved Layer State | las |  |
| 894 | x-fmt/60 | AutoCAD Linetype Definition File | lin |  |
| 895 | x-fmt/70 | AutoCAD Menu Resource File | mnr, mnt |  |
| 896 | x-fmt/107 | AutoCAD Named Plot Style Table | stb |  |
| 897 | x-fmt/79 | AutoCAD Plot Configuration File | pcp |  |
| 898 | x-fmt/77 | AutoCAD Plot Configuration File | pc2 |  |
| 899 | x-fmt/78 | AutoCAD Plot Configuration File | pc3 |  |
| 900 | x-fmt/100 | AutoCAD Script | scr |  |
| 901 | x-fmt/105 | AutoCAD Slide | sld | application/sld, application/x-sld, image/x-sld |
| 902 | x-fmt/104 | AutoCAD Slide Library | slb |  |
| 903 | x-fmt/71 | AutoCAD Source Menu File | mns |  |
| 904 | x-fmt/72 | AutoCAD Template Menu File | mnu |  |
| 905 | fmt/1257 | AutoCAD Temporary File | ac$ |  |
| 906 | x-fmt/127 | AutoCAD Xref Log | xlg |  |
| 907 | fmt/1916 | Autodesk Alias Wire Format |  |  |
| 908 | fmt/299 | Autodesk Animator (FlicLib) | fli |  |
| 909 | x-fmt/223 | Autodesk Animator CEL File Format | cel |  |
| 910 | fmt/298 | Autodesk Animator Pro FLIC | flc |  |
| 911 | x-fmt/154 | AutoDesk FLIC Animation | fli |  |
| 912 | fmt/1562 | AutoDesk Indexed Point Cloud | pcg |  |
| 913 | fmt/1349 | Autodesk Revit Family File | rfa, rft |  |
| 914 | fmt/1351 | Autodesk Revit Family File | rfa, rft |  |
| 915 | fmt/1348 | Autodesk Revit Family File | rfa, rft |  |
| 916 | fmt/1346 | Autodesk Revit File | rvt, rfa, rte, rft |  |
| 917 | fmt/1347 | Autodesk Revit Project File | rvt, rte, rft |  |
| 918 | fmt/1350 | Autodesk Revit Project File | rvt, rte |  |
| 919 | linguist/26 | AutoHotkey | .ahk, .ahkl |  |
| 920 | linguist/27 | AutoIt | .au3 |  |
| 921 | x-fmt/63 | AutoLISP File | lsp |  |
| 922 | x-fmt/69 | AutoLISP Menu Source File | mnl |  |
| 923 | fmt/331 | Autorun Configuration File | inf |  |
| 924 | fmt/1461 | Autorun Maestro Menu File | mnu |  |
| 925 | fmt/1044 | AutoShade Rendering Slide | rnd |  |
| 926 | x-fmt/306 | AutoSketch Drawing | skf |  |
| 927 | fmt/1054 | AVCHD Clip Information File | cpi, clpi |  |
| 928 | fmt/1076 | AVCHD Index File | bdm, bdmv |  |
| 929 | fmt/1075 | AVCHD Movie Object File | bdm, bdmv |  |
| 930 | fmt/1074 | AVCHD Playlist File | mpl, mpls |  |
| 931 | fmt/1077 | AVCHD Thumbnail Index File | tid |  |
| 932 | fmt/1331 | Avery DesignPro Document | zdl |  |
| 933 | fmt/1330 | Avery DesignPro Document | zdp |  |
| 934 | fmt/1329 | Avery Label Pro Document | lpd |  |
| 935 | linguist/785497837 | Avro IDL | .avdl |  |
| 936 | fmt/1179 | Away3D Data Format | awd |  |
| 937 | linguist/28 | Awk | .awk, .auk, .gawk, .mawk, .nawk |  |
| 938 | fmt/884 | AXD HTTP Handler File | axd |  |
| 939 | fmt/1804 | B Source Code File | b |  |
| 940 | linguist/96642275 | B4X | .bas | text/x-vb |
| 941 | fmt/941 | Back Up File | bak |  |
| 942 | linguist/720859680 | Ballerina | .bal |  |
| 943 | fmt/1238 | Band Interleaved By Line (BIL) Image Encoding | bil |  |
| 944 | fmt/1239 | Band Interleaved By Pixel (BIP) Image Encoding | bip |  |
| 945 | fmt/1240 | Band Sequential (BSQ) Image Encoding | bsq |  |
| 946 | linguist/28923963 | BASIC | .bas |  |
| 947 | fmt/885 | BASIC File | bas |  |
| 948 | x-fmt/413 | Batch file (executable) | bat |  |
| 949 | linguist/29 | Batchfile | .bat, .cmd |  |
| 950 | fmt/1650 | Bayesian Interchange Format File | bif |  |
| 951 | fmt/1342 | BDOC | bdoc, asice | application/vnd.etsi.asic-e+zip |
| 952 | fmt/1340 | BDOC | bdoc | application/vnd.bdoc-1.0 |
| 953 | fmt/1559 | Beam Software SIFF File | son, vb |  |
| 954 | linguist/545626333 | Beef | .bf | text/x-csharp |
| 955 | linguist/30 | Befunge | .befunge, .bf |  |
| 956 | fmt/1549 | Bentley Microstation Hidden Line File | hln |  |
| 957 | fmt/502 | Bentley V8 DGN | dgn |  |
| 958 | linguist/121855308 | Berry | .be |  |
| 959 | fmt/687 | Better Portable Graphics | bpg |  |
| 960 | linguist/982188347 | BibTeX | .bib, .bibtex | text/x-stex |
| 961 | fmt/1616 | BibTeX Database File | bib |  |
| 962 | linguist/321200902 | Bicep | .bicep, .bicepparam |  |
| 963 | fmt/1917 | BigTIFF | tif, tf8, btf |  |
| 964 | linguist/1055528081 | Bikeshed | .bs | text/html |
| 965 | fmt/1722 | BIM Metadata File | bim |  |
| 966 | default/2 | Binary |  | application/octet-stream |
| 967 | fmt/208 | Binary File | bin |  |
| 968 | fmt/984 | Binary Property List | plist, nib, aae, iMovieProj, ezdraw |  |
| 969 | x-fmt/416 | BinHex Binary Text | hqx | application/mac-binhex40 |
| 970 | fmt/731 | Bink Video Format | bik |  |
| 971 | fmt/732 | Bink Video Format | bik2, bk2 | video/vnd.radgamettools.bink |
| 972 | linguist/31 | Bison | .bison |  |
| 973 | linguist/32 | BitBake | .bb, .bbappend, .bbclass, .inc |  |
| 974 | fmt/1569 | Bitstream Speedo Fonts | spd |  |
| 975 | fmt/1052 | BKNAS Seismic Data Format | bknas |  |
| 976 | linguist/33 | Blade | .blade, .blade.php |  |
| 977 | fmt/902 | Blender 3D | blend |  |
| 978 | fmt/903 | Blender 3D | blend |  |
| 979 | fmt/1182 | Blitz3D File Format | b3d |  |
| 980 | linguist/34 | BlitzBasic | .bb, .decls |  |
| 981 | linguist/35 | BlitzMax | .bmx |  |
| 982 | linguist/36 | Bluespec | .bsv | text/x-systemverilog |
| 983 | linguist/641580358 | Bluespec BH | .bs | text/x-haskell |
| 984 | fmt/904 | Bluetooth Snoop Packet Capture | log |  |
| 985 | fmt/1181 | Bodypaint 3D | b3d |  |
| 986 | linguist/37 | Boo | .boo |  |
| 987 | linguist/955017407 | Boogie | .bpl |  |
| 988 | fmt/393 | Borland Reflex flat datafile | rxd |  |
| 989 | linguist/330386870 | BQN | .bqn |  |
| 990 | linguist/38 | Brainfuck | .b, .bf | text/x-brainfuck |
| 991 | linguist/943571030 | BrighterScript | .bs |  |
| 992 | linguist/39 | Brightscript | .brs |  |
| 993 | fmt/1836 | Brio Query File | bqy |  |
| 994 | fmt/518 | Broad Band eBook | lrf |  |
| 995 | fmt/711 | Broadcast WAVE | wav | audio/x-wav |
| 996 | fmt/703 | Broadcast WAVE | wav | audio/x-wav |
| 997 | fmt/706 | Broadcast WAVE | wav | audio/x-wav |
| 998 | fmt/1 | Broadcast WAVE | wav | audio/x-wav |
| 999 | fmt/705 | Broadcast WAVE | wav | audio/x-wav |
| 1000 | fmt/704 | Broadcast WAVE | wav | audio/x-wav |
| 1001 | fmt/710 | Broadcast WAVE | wav, rf64 | audio/x-wav |
| 1002 | fmt/527 | Broadcast WAVE | wav | audio/x-wav |
| 1003 | fmt/707 | Broadcast WAVE | wav | audio/x-wav |
| 1004 | fmt/2 | Broadcast WAVE | wav | audio/x-wav |
| 1005 | fmt/709 | Broadcast WAVE | wav, rf64 | audio/x-wav |
| 1006 | fmt/708 | Broadcast WAVE | wav | audio/x-wav |
| 1007 | x-fmt/168 | Broderbund Print Shop Deluxe | pcc, pdb, pdc, pda, pdl, pds, pdg |  |
| 1008 | fmt/1299 | Broderbund Print Shop Deluxe | pdb, pds, pcb, pdc, pcc, pce, pdg, pdl, pso, pdp, pho, pcp, ppi, pda |  |
| 1009 | fmt/1300 | Broderbund The Print Shop/PrintMaster/American Greetings Project | ban, bro, biz, cal, car, cer, env, fax, sig, cft, hcr, lbl, let, nws, not, pcr, php, tsh, web, sti |  |
| 1010 | linguist/153503348 | Browserslist |  |  |
| 1011 | fmt/1385 | Bruker PDZ | pdz, xpdz |  |
| 1012 | fmt/439 | BSDIFF | bsdiff |  |
| 1013 | x-fmt/308 | Btrieve Database | btr |  |
| 1014 | x-fmt/267 | BZIP Compressed Archive | bz |  |
| 1015 | x-fmt/268 | BZIP2 Compressed Archive | bz2 | application/x-bzip2 |
| 1016 | linguist/41 | C | .c, .cats, .h, .idc | text/x-csrc |
| 1017 | fmt/1768 | C Source Code File | c |  |
| 1018 | linguist/42 | C# | .cs, .cake, .cs.pp, .csx, .linq | text/x-csharp |
| 1019 | linguist/43 | C++ | .cpp, .c++, .cc, .cp, .cppm, .cxx, .h, .h++, .hh, .hpp, .hxx, .inc, .inl, .ino, .ipp, .ixx, .re, .tcc, .tpp, .txx | text/x-c++src |
| 1020 | fmt/1769 | C++ Source Code File | cpp, cxx, cc |  |
| 1021 | linguist/44 | C-ObjDump | .c-objdump |  |
| 1022 | fmt/1735 | C/C++ Header File | h, hpp, hxx |  |
| 1023 | linguist/45 | C2hs Haskell | .chs | text/x-haskell |
| 1024 | fmt/1130 | C3D File Format | c3d |  |
| 1025 | linguist/677095381 | Cabal Config | .cabal | text/x-haskell |
| 1026 | linguist/615465151 | Caddyfile | .caddyfile |  |
| 1027 | linguist/270184138 | Cadence | .cdc |  |
| 1028 | linguist/620599567 | Cairo | .cairo |  |
| 1029 | linguist/891399890 | Cairo Zero | .cairo |  |
| 1030 | fmt/1214 | Cakewalk WRK Project | wrk |  |
| 1031 | fmt/1712 | Calc602 Macro File | mc6 |  |
| 1032 | fmt/1767 | Calc602 Project File | pc6 |  |
| 1033 | fmt/1775 | Calc602 Project File | pc6 |  |
| 1034 | fmt/1713 | Calc602 Project File | pc6 |  |
| 1035 | fmt/1697 | Calc602 Spreadsheet file | bak, tc6 |  |
| 1036 | fmt/1773 | Calc602 Spreadsheet File | bak, tc6 |  |
| 1037 | fmt/1698 | Calc602 Spreadsheet file | bak, tc6 |  |
| 1038 | fmt/1295 | Calendar Creator Event | ce3 |  |
| 1039 | fmt/1297 | Calendar Creator File | cc5 |  |
| 1040 | fmt/1298 | Calendar Creator File | bcc |  |
| 1041 | fmt/1296 | Calendar Creator File | cc3 |  |
| 1042 | x-fmt/141 | Calendar Creator Plus Data File | cce |  |
| 1043 | fmt/914 | Caligari trueSpace File Format | cob, scn |  |
| 1044 | fmt/913 | Caligari trueSpace File Format | cob, scn |  |
| 1045 | x-fmt/28 | CALS Compressed Bitmap | cal |  |
| 1046 | linguist/829207807 | CameLIGO | .mligo | text/x-ocaml |
| 1047 | fmt/1852 | Camtasia Recording File | camrec |  |
| 1048 | fmt/1853 | Camtasia Studio Project | camproj |  |
| 1049 | fmt/1750 | Canon CIF File | cif |  |
| 1050 | fmt/1749 | Canon MIF File | mif |  |
| 1051 | fmt/592 | Canon RAW | cr2 |  |
| 1052 | fmt/593 | Canon RAW | crw |  |
| 1053 | fmt/1595 | Canon Raw | cr3 |  |
| 1054 | fmt/1751 | Canon SIF File | sif |  |
| 1055 | linguist/390788699 | CAP CDS | .cds |  |
| 1056 | linguist/52 | Cap'n Proto | .capnp |  |
| 1057 | fmt/1857 | Capture One Session File | cos |  |
| 1058 | fmt/1725 | Capture One Settings File | cos |  |
| 1059 | linguist/55627273 | Carbon | .carbon | text/x-go |
| 1060 | fmt/1254 | Cardfile | crd |  |
| 1061 | fmt/727 | Cartesian Perceptual Compression image format | cpi, cpc |  |
| 1062 | linguist/53 | CartoCSS | .mss |  |
| 1063 | x-fmt/224 | Cascading Style Sheet | css | text/css |
| 1064 | fmt/1772 | Casio QV CAM | cam |  |
| 1065 | fmt/1615 | CATIA Drawing | catdrawing |  |
| 1066 | x-fmt/438 | CATIA Material Description | catmaterial |  |
| 1067 | x-fmt/436 | CATIA Model | mod, model |  |
| 1068 | x-fmt/439 | CATIA Model (Part Description) | catpart |  |
| 1069 | fmt/1714 | CATIA Model File | model |  |
| 1070 | x-fmt/440 | CATIA Product Description | catproduct |  |
| 1071 | x-fmt/437 | CATIA Project | project |  |
| 1072 | x-fmt/201 | CCITT G.711 Audio | ulaw |  |
| 1073 | x-fmt/222 | CD Audio | cda | application/x-cdf |
| 1074 | fmt/819 | CD-ROM/XA (eXtended Architecture) | dat |  |
| 1075 | fmt/1655 | cdrLabel Label File | clb |  |
| 1076 | fmt/869 | CDX Internet Archive Index | cdx |  |
| 1077 | linguist/54 | Ceylon | .ceylon |  |
| 1078 | linguist/55 | Chapel | .chpl |  |
| 1079 | linguist/56 | Charity | .ch |  |
| 1080 | fmt/665 | Chasys Draw image file | cd5 |  |
| 1081 | fmt/1798 | CHAT Transcription Format | cha | text/x-chat |
| 1082 | linguist/372063053 | Checksums | .crc32, .md2, .md4, .md5, .sha1, .sha2, .sha224, .sha256, .sha256sum, .sha3, .sha384, .sha512 |  |
| 1083 | fmt/378 | Chemical Draw Exchange Format | cdx | chemical/x-cdx |
| 1084 | fmt/333 | Chemical Markup Language | cml |  |
| 1085 | apache-httpd/1407925950950459349 | chemical/x-cif | cif | chemical/x-cif |
| 1086 | apache-httpd/4276800422078512688 | chemical/x-cmdf | cmdf | chemical/x-cmdf |
| 1087 | apache-httpd/15268548954167723934 | chemical/x-cml | cml | chemical/x-cml |
| 1088 | apache-httpd/7238172098370228130 | chemical/x-csml | csml | chemical/x-csml |
| 1089 | apache-httpd/235929493062174822 | chemical/x-xyz | xyz | chemical/x-xyz |
| 1090 | x-fmt/309 | ChiWriter Document | chi |  |
| 1091 | fmt/300 | ChiWriter Document | chi |  |
| 1092 | linguist/57 | ChucK | .ck | text/x-java |
| 1093 | linguist/29176339 | CIL | .cil |  |
| 1094 | fmt/1278 | Cindex Document | ucdx, utpl |  |
| 1095 | fmt/1277 | Cindex Document | cdx, tpl |  |
| 1096 | fmt/1279 | Cindex Document | ucdx, utpl |  |
| 1097 | fmt/1180 | Cinema 4D | c4d |  |
| 1098 | fmt/540 | Cinema 4D | c4d |  |
| 1099 | fmt/415 | Cinema 4D | c4d |  |
| 1100 | fmt/1716 | Cintel Raw Image/DaVinci Resolve Image | cri, dvcc |  |
| 1101 | linguist/1042332086 | Circom | .circom |  |
| 1102 | linguist/58 | Cirru | .cirru |  |
| 1103 | linguist/59 | Clarion | .clw |  |
| 1104 | fmt/736 | ClarisWorks | cwk |  |
| 1105 | fmt/737 | ClarisWorks | cwk |  |
| 1106 | fmt/848 | ClarisWorks Database | cwk |  |
| 1107 | fmt/741 | ClarisWorks Database | cwk |  |
| 1108 | fmt/845 | ClarisWorks Drawing | cwk |  |
| 1109 | fmt/738 | ClarisWorks Drawing | cwk |  |
| 1110 | fmt/849 | ClarisWorks Painting | cwk |  |
| 1111 | fmt/742 | ClarisWorks Painting | cwk |  |
| 1112 | fmt/740 | ClarisWorks Spreadsheet | cwk |  |
| 1113 | fmt/847 | ClarisWorks Spreadsheet | cwk |  |
| 1114 | fmt/739 | ClarisWorks Word Processor | cwk |  |
| 1115 | fmt/846 | ClarisWorks Word Processor | cwk |  |
| 1116 | fmt/746 | ClarisWorks/AppleWorks Database | cwk |  |
| 1117 | fmt/743 | ClarisWorks/AppleWorks Drawing | cwk |  |
| 1118 | fmt/747 | ClarisWorks/AppleWorks Painting | cwk |  |
| 1119 | fmt/745 | ClarisWorks/AppleWorks Spreadsheet | cwk |  |
| 1120 | fmt/744 | ClarisWorks/AppleWorks Word Processor | cwk |  |
| 1121 | linguist/91493841 | Clarity | .clar |  |
| 1122 | linguist/8 | Classic ASP | .asp |  |
| 1123 | linguist/60 | Clean | .icl, .dcl |  |
| 1124 | linguist/61 | Click | .click |  |
| 1125 | linguist/46 | CLIPS | .clp |  |
| 1126 | linguist/62 | Clojure | .clj, .bb, .boot, .cl2, .cljc, .cljs, .cljs.hl, .cljscm, .cljx, .hic | text/x-clojure |
| 1127 | fmt/1760 | CloneCD Control File | ccd |  |
| 1128 | linguist/357046146 | Closure Templates | .soy | text/x-soy |
| 1129 | linguist/407996372 | Cloud Firestore Security Rules |  | text/css |
| 1130 | fmt/1885 | CloudCompare Entity File | bin |  |
| 1131 | linguist/47 | CMake | .cmake, .cmake.in | text/x-cmake |
| 1132 | linguist/48 | COBOL | .cob, .cbl, .ccp, .cobol, .cpy | text/x-cobol |
| 1133 | linguist/321684729 | CODEOWNERS |  |  |
| 1134 | linguist/424259634 | CodeQL | .ql, .qll |  |
| 1135 | linguist/63 | CoffeeScript | .coffee, ._coffee, .cake, .cjsx, .iced | text/x-coffeescript |
| 1136 | fmt/1587 | COKE Format (Atari Falcon) | tg1 |  |
| 1137 | linguist/64 | ColdFusion | .cfm, .cfml |  |
| 1138 | linguist/65 | ColdFusion CFC | .cfc |  |
| 1139 | fmt/1566 | ColdFusion Markup Language | cfm |  |
| 1140 | linguist/49 | COLLADA | .dae | text/xml |
| 1141 | fmt/1209 | COLLADA Digital Asset Exchange (DAE) | dae | model/vnd.collada+xml |
| 1142 | fmt/1462 | Comic Book Archive | cb7, cba, cbr, cbt, cbz |  |
| 1143 | x-fmt/18 | Comma Separated Values | csv | text/csv |
| 1144 | fmt/1950 | Common Data Format dotCDF | cdf |  |
| 1145 | fmt/1949 | Common Data Format dotCDF | cdf |  |
| 1146 | fmt/1948 | Common Data Format dotCDF | cdf |  |
| 1147 | fmt/1888 | Common Instrument File (CIF) | ci2 |  |
| 1148 | fmt/1887 | Common Instrument File (CIF) | ci1 |  |
| 1149 | fmt/1871 | Common Interface File | cif, mca |  |
| 1150 | linguist/66 | Common Lisp | .lisp, .asd, .cl, .l, .lsp, .ny, .podsl, .sexp | text/x-common-lisp |
| 1151 | fmt/1945 | Common Loudspeaker Format (CLF) | cf2 |  |
| 1152 | fmt/1944 | Common Loudspeaker Format (CLF) | cf1 |  |
| 1153 | linguist/988547172 | Common Workflow Language | .cwl | text/x-yaml |
| 1154 | linguist/67 | Component Pascal | .cp, .cps | text/x-pascal |
| 1155 | fmt/892 | Compound WordPerfect for Windows Document | wpd, doc, wp6, wp, w60 | application/vnd.wordperfect |
| 1156 | fmt/897 | Compressed MusicXML | mxl | application/vnd.recordare.musicxml |
| 1157 | fmt/2005 | Compressed MusicXML | mxl | application/vnd.recordare.musicxml |
| 1158 | fmt/1538 | CompuServe RLE | rle |  |
| 1159 | fmt/1144 | CompuServe WinCIM Message Format | plx, msg |  |
| 1160 | fmt/305 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=3 |
| 1161 | fmt/304 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=2 |
| 1162 | fmt/303 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=1 |
| 1163 | fmt/306 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=4 |
| 1164 | fmt/301 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| 1165 | x-fmt/142 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| 1166 | fmt/302 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| 1167 | linguist/421026389 | CoNLL-U | .conllu, .conll |  |
| 1168 | linguist/68 | Cool | .cl |  |
| 1169 | fmt/1498 | Cool Edit/Adobe Audition Session File | ses |  |
| 1170 | linguist/69 | Coq | .coq, .v |  |
| 1171 | fmt/1413 | Corel Gallery Clipart | bmf |  |
| 1172 | fmt/1422 | Corel Photo House Image | cps |  |
| 1173 | x-fmt/144 | Corel Photo-Paint Image | cpt |  |
| 1174 | fmt/877 | Corel Presentation | shw |  |
| 1175 | fmt/878 | Corel Presentation | shw |  |
| 1176 | x-fmt/35 | Corel Presentation Exchange File | cmx |  |
| 1177 | x-fmt/34 | Corel Presentation Exchange File | cmx |  |
| 1178 | fmt/1418 | Corel Print House Document | cph, cpd |  |
| 1179 | fmt/1417 | Corel Print House Document | cph, cpd |  |
| 1180 | fmt/1420 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| 1181 | fmt/1419 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| 1182 | fmt/1421 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| 1183 | x-fmt/33 | Corel R.A.V.E. | clk |  |
| 1184 | fmt/432 | Corel R.A.V.E. | clk |  |
| 1185 | fmt/431 | Corel R.A.V.E. | clk |  |
| 1186 | x-fmt/202 | Corel Wavelet Compressed Bitmap | wi, wvl |  |
| 1187 | fmt/1313 | CorelCHART Document | cch |  |
| 1188 | fmt/1312 | CorelCHART Document | cch |  |
| 1189 | x-fmt/310 | CorelCHART Document | cch |  |
| 1190 | x-fmt/31 | CorelDraw Compressed Drawing | cdx, cjw |  |
| 1191 | x-fmt/36 | CorelDraw Compressed Drawing | cpx |  |
| 1192 | fmt/465 | CorelDraw Drawing | cdr |  |
| 1193 | fmt/1930 | CorelDraw Drawing | cdr |  |
| 1194 | fmt/466 | CorelDraw Drawing | cdr |  |
| 1195 | fmt/1928 | CorelDraw Drawing | cdr |  |
| 1196 | x-fmt/375 | CorelDraw Drawing | cdr |  |
| 1197 | x-fmt/379 | CorelDraw Drawing | cdr |  |
| 1198 | fmt/1932 | CorelDraw Drawing | cdr |  |
| 1199 | fmt/430 | CorelDraw Drawing | cdr |  |
| 1200 | x-fmt/378 | CorelDraw Drawing | cdr |  |
| 1201 | fmt/1927 | CorelDraw Drawing | cdr |  |
| 1202 | fmt/1929 | CorelDraw Drawing | cdr |  |
| 1203 | fmt/467 | CorelDraw Drawing | cdr |  |
| 1204 | fmt/1925 | CorelDraw Drawing | cdr |  |
| 1205 | fmt/1926 | CorelDraw Drawing | cdr |  |
| 1206 | x-fmt/292 | CorelDraw Drawing | cdr |  |
| 1207 | fmt/429 | CorelDraw Drawing | cdr |  |
| 1208 | fmt/1931 | CorelDraw Drawing | cdr |  |
| 1209 | fmt/1933 | CorelDraw Drawing | cdr |  |
| 1210 | fmt/464 | CorelDraw Drawing | cdr |  |
| 1211 | fmt/427 | CorelDraw Drawing | cdr |  |
| 1212 | x-fmt/374 | CorelDraw Drawing | cdr |  |
| 1213 | fmt/1924 | CorelDraw Drawing | cdr |  |
| 1214 | fmt/428 | CorelDraw Drawing | cdr |  |
| 1215 | x-fmt/291 | CorelDraw Drawing | cdr |  |
| 1216 | x-fmt/29 | CorelDraw Drawing | cdr |  |
| 1217 | fmt/1934 | CorelDraw Drawing | cdr |  |
| 1218 | x-fmt/76 | CorelDraw Pattern | pat |  |
| 1219 | x-fmt/30 | CorelDraw Template | cdt |  |
| 1220 | fmt/1676 | Covox ADPCM Audio Files | v8, cvx, v2s, v3s, v4s, vmf |  |
| 1221 | fmt/635 | CPIO | cpio |  |
| 1222 | linguist/70 | Cpp-ObjDump | .cppobjdump, .c++-objdump, .c++objdump, .cpp-objdump, .cxx-objdump |  |
| 1223 | fmt/910 | CRAM File Format | cram |  |
| 1224 | fmt/909 | CRAM File Format | cram |  |
| 1225 | fmt/911 | CRAM File Format | cram |  |
| 1226 | fmt/1736 | Creative Voice File | voc |  |
| 1227 | linguist/71 | Creole | .creole |  |
| 1228 | linguist/705203557 | crontab |  |  |
| 1229 | fmt/822 | CRT C64 Cartridge Image Format | crt |  |
| 1230 | linguist/72 | Crystal | .cr | text/x-crystal |
| 1231 | fmt/1648 | Crystal Reports File | rpt | application/x-rpt |
| 1232 | fmt/334 | Crystallographic Information Framework | cif |  |
| 1233 | linguist/424 | CSON | .cson | text/x-coffeescript |
| 1234 | linguist/73 | Csound | .orc, .udo |  |
| 1235 | linguist/74 | Csound Document | .csd |  |
| 1236 | linguist/75 | Csound Score | .sco |  |
| 1237 | linguist/50 | CSS | .css | text/css |
| 1238 | linguist/51 | CSV | .csv |  |
| 1239 | fmt/800 | CSV Schema | csvs | text/csv-schema |
| 1240 | linguist/77 | Cuda | .cu, .cuh | text/x-c++src |
| 1241 | linguist/356063509 | CUE | .cue |  |
| 1242 | fmt/1069 | Cue Sheet | cue |  |
| 1243 | linguist/942714150 | Cue Sheet | .cue |  |
| 1244 | linguist/992375436 | cURL Config |  |  |
| 1245 | linguist/439829048 | Curry | .curry |  |
| 1246 | linguist/657332628 | CWeb | .w |  |
| 1247 | fmt/1557 | Cyber Paint Sequence | seq |  |
| 1248 | linguist/78 | Cycript | .cy | text/javascript |
| 1249 | linguist/476447814 | Cylc | .cylc |  |
| 1250 | linguist/850806976 | Cypher | .cyp, .cypher |  |
| 1251 | fmt/658 | Cypher Query Language | cql |  |
| 1252 | linguist/79 | Cython | .pyx, .pxd, .pxi | text/x-cython |
| 1253 | linguist/80 | D | .d, .di | text/x-d |
| 1254 | linguist/81 | D-ObjDump | .d-objdump |  |
| 1255 | linguist/37531557 | D2 | .d2 |  |
| 1256 | linguist/969323346 | Dafny | .dfy |  |
| 1257 | fmt/1547 | Daisy-Dot Font File | nlq |  |
| 1258 | fmt/1546 | Daisy-Dot Font File | nlq |  |
| 1259 | fmt/694 | Dalvik Executable Format | dex |  |
| 1260 | linguist/86 | Darcs Patch | .darcspatch, .dpatch |  |
| 1261 | linguist/87 | Dart | .dart | application/dart |
| 1262 | fmt/1730 | Data File | dat |  |
| 1263 | x-fmt/41 | Data Interchange Format | dif |  |
| 1264 | x-fmt/197 | DataFlex Query Tag Name | tag |  |
| 1265 | linguist/974514097 | DataWeave | .dwl |  |
| 1266 | fmt/1851 | DAV Video Format | dav |  |
| 1267 | x-fmt/8 | dBASE Database | dbf |  |
| 1268 | x-fmt/272 | dBASE Database | dbf |  |
| 1269 | x-fmt/9 | dBASE Database | dbf |  |
| 1270 | x-fmt/10 | dBASE Database | dbf | application/dbase |
| 1271 | x-fmt/271 | dBASE Database | dbf |  |
| 1272 | x-fmt/380 | dBASE for Windows database | dbf |  |
| 1273 | fmt/1860 | dBASE Report Form Definition File | frm |  |
| 1274 | x-fmt/311 | dBASE Text Memo | dbt |  |
| 1275 | fmt/1728 | dBASE Windows Form File | wfm |  |
| 1276 | linguist/527438264 | Debian Package Control File | .dsc |  |
| 1277 | fmt/1365 | Debug File | dbg |  |
| 1278 | x-fmt/286 | DEC Data Exchange File | dx | application/dec-dx. |
| 1279 | x-fmt/287 | DEC WPS Plus Document | wpl |  |
| 1280 | x-fmt/424 | Deluxe Paint bitmap | lbm |  |
| 1281 | fmt/1363 | DeluxePaint Animation File | anm |  |
| 1282 | linguist/435000929 | DenizenScript | .dsc | text/x-yaml |
| 1283 | x-fmt/312 | DesignCAD Drawing | dc2, dc |  |
| 1284 | x-fmt/313 | DesignCAD for Windows Drawing | dw2 |  |
| 1285 | linguist/412 | desktop | .desktop, .desktop.in, .service |  |
| 1286 | x-fmt/149 | Desktop Color Separation File | dcs |  |
| 1287 | fmt/1617 | Devicetree Blob (DTB) | dtb |  |
| 1288 | linguist/793969321 | Dhall | .dhall | text/x-haskell |
| 1289 | x-fmt/381 | Dia Graphics Format | dia |  |
| 1290 | linguist/88 | Diff | .diff, .patch | text/x-diff |
| 1291 | fmt/1121 | DIFFRACplus Raw Data File Format | raw |  |
| 1292 | fmt/1120 | DIFFRACplus Raw Data File Format | raw |  |
| 1293 | fmt/1232 | DIGIDOC-XML | ddoc |  |
| 1294 | fmt/1233 | DIGIDOC-XML | ddoc |  |
| 1295 | fmt/1231 | DIGIDOC-XML | ddoc |  |
| 1296 | linguist/82 | DIGITAL Command Language | .com |  |
| 1297 | fmt/574 | Digital Imaging and Communications in Medicine File Format | dcm | application/dicom |
| 1298 | fmt/193 | Digital Moving Picture Exchange Bitmap | dpx |  |
| 1299 | fmt/541 | Digital Moving Picture Exchange Bitmap | dpx |  |
| 1300 | fmt/152 | Digital Negative Format (DNG) | dng, tif, tiff | image/dng, image/tiff |
| 1301 | fmt/1943 | Digital Negative Format (DNG) | dng | image/dng |
| 1302 | fmt/438 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| 1303 | fmt/1841 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| 1304 | fmt/730 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| 1305 | fmt/437 | Digital Negative Format (DNG) | dng | image/dng |
| 1306 | fmt/1842 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| 1307 | fmt/436 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| 1308 | fmt/1007 | Digital Speech Standard | dss |  |
| 1309 | x-fmt/314 | Digital Terrain Elevation Data | dted, dt0, dt1, dt2, avg, min, max |  |
| 1310 | x-fmt/152 | Digital Video | dv | video/dv |
| 1311 | fmt/1891 | Digital Voice File (DVF) | dvf |  |
| 1312 | linguist/691605112 | dircolors | .dircolors |  |
| 1313 | fmt/1818 | Direct Stream Digital Interchange File Format | dff |  |
| 1314 | fmt/1817 | Direct Stream Digital Stream File | dsf |  |
| 1315 | fmt/1040 | DirectDraw Surface | dds |  |
| 1316 | fmt/957 | DirectMusic Segment File Format | sgt |  |
| 1317 | fmt/958 | DirectMusic Style File Format | sty |  |
| 1318 | linguist/201049282 | DirectX 3D File | .x |  |
| 1319 | fmt/1399 | DiskDoubler |  |  |
| 1320 | fmt/1960 | Disklavier E-Seq Music | fil, esq |  |
| 1321 | fmt/255 | DjVu File Format | djvu, djv | image/vnd.djvu, image/x-djvu |
| 1322 | linguist/83 | DM | .dm |  |
| 1323 | fmt/1554 | DNA Sequence Chromatogram File | scf |  |
| 1324 | linguist/84 | DNS Zone | .zone, .arpa |  |
| 1325 | linguist/89 | Dockerfile | .dockerfile, .containerfile | text/x-dockerfile |
| 1326 | x-fmt/315 | Document Type Definition | dtd |  |
| 1327 | fmt/1827 | DOCX Strict OOXML Document | docx | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| 1328 | linguist/90 | Dogescript | .djs |  |
| 1329 | fmt/735 | Dolby Digital AC-3 | ac3 | audio/ac3 |
| 1330 | fmt/972 | Dolby MLP Lossless Audio | mlp | audio/vnd.dolby.mlp |
| 1331 | fmt/572 | Domino XML Database Export | dxl |  |
| 1332 | fmt/571 | Domino XML Document Export | dxl |  |
| 1333 | fmt/960 | DOS Sound and Music Interface Advanced Module Format | amf |  |
| 1334 | linguist/111148035 | Dotenv | .env |  |
| 1335 | fmt/955 | Downloadable Sounds Audio | dls | audio/dls |
| 1336 | x-fmt/316 | Dr Halo Bitmap | cut |  |
| 1337 | fmt/1186 | Dr. Halo Image Palette | pal |  |
| 1338 | fmt/1046 | Draco File Format | drc |  |
| 1339 | fmt/1946 | Draw.io Diagram (XML) File | drawio, xml |  |
| 1340 | fmt/54 | Drawing Interchange Binary Format | dxb |  |
| 1341 | fmt/66 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1342 | fmt/73 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1343 | fmt/69 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1344 | fmt/76 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1345 | fmt/63 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1346 | fmt/532 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1347 | fmt/433 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1348 | fmt/68 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1349 | fmt/71 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1350 | fmt/74 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1351 | fmt/79 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1352 | fmt/64 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1353 | fmt/435 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1354 | fmt/67 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1355 | fmt/65 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1356 | fmt/78 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1357 | fmt/72 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1358 | fmt/70 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1359 | fmt/75 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1360 | fmt/77 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| 1361 | fmt/81 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1362 | fmt/84 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1363 | fmt/83 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1364 | fmt/80 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1365 | fmt/82 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1366 | fmt/85 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| 1367 | fmt/1389 | Drawing Interchange Format (ASCII) | dxf | image/vnd.dxf |
| 1368 | fmt/1393 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| 1369 | fmt/1391 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| 1370 | fmt/1394 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| 1371 | fmt/1392 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| 1372 | fmt/1390 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| 1373 | x-fmt/52 | Drawing Interchange Format Style Extract | dxx |  |
| 1374 | fmt/335 | Dreamweaver Lock File | lck |  |
| 1375 | fmt/120 | DROID File Collection File Format | xml | text/xml |
| 1376 | fmt/121 | DROID Signature File Format | xml | text/xml |
| 1377 | fmt/394 | DS_Store File (MAC) | ds_store |  |
| 1378 | fmt/1008 | DSS Pro | ds2 |  |
| 1379 | linguist/85 | DTrace | .d | text/x-csrc |
| 1380 | fmt/973 | DTS Coherent Acoustics (DCA) Audio | dts | audio/vnd.dts |
| 1381 | linguist/754574151 | Dune |  |  |
| 1382 | x-fmt/419 | DVD data file and backup data file | ifo, bup |  |
| 1383 | linguist/91 | Dylan | .dylan, .dyl, .intr, .lid | text/x-dylan |
| 1384 | fmt/1779 | Dynamic Publisher Font File | fnt |  |
| 1385 | fmt/1778 | Dynamic Publisher Picture File | pct |  |
| 1386 | linguist/92 | E | .e |  |
| 1387 | linguist/529653389 | E-mail | .eml, .mbox | application/mbox |
| 1388 | linguist/97 | Eagle | .sch, .brd | text/xml |
| 1389 | fmt/372 | Earth Resource Satellite Image Header Format | ers |  |
| 1390 | linguist/963512632 | Earthly |  |  |
| 1391 | fmt/1665 | Easy CD Creator Layout | Roxio Easy CD Creator Layout | rcl, cl5 |  |
| 1392 | linguist/342840477 | Easybuild | .eb | text/x-python |
| 1393 | fmt/981 | EazyDraw File Format | ezdraw |  |
| 1394 | fmt/159 | EBCDIC-US | ebcdic |  |
| 1395 | linguist/430 | EBNF | .ebnf | text/x-ebnf |
| 1396 | fmt/1940 | EBU Subtitling Data Exchange Format | stl |  |
| 1397 | linguist/413 | eC | .ec, .eh |  |
| 1398 | linguist/98 | Ecere Projects | .epj | application/json |
| 1399 | linguist/93 | ECL | .ecl, .eclxml | text/x-ecl |
| 1400 | linguist/94 | ECLiPSe | .ecl |  |
| 1401 | fmt/1235 | EclipseCrossword Puzzle File | ecw |  |
| 1402 | fmt/1236 | EclipseCrossword Word List File | ewl |  |
| 1403 | linguist/844766630 | Ecmarkup | .html | text/html |
| 1404 | linguist/460509620 | Edge | .edge |  |
| 1405 | linguist/925235833 | EdgeQL | .edgeql, .esdl |  |
| 1406 | linguist/96139566 | EditorConfig | .editorconfig | text/x-properties |
| 1407 | linguist/342840478 | Edje Data Collection | .edc | text/x-c++src |
| 1408 | linguist/414 | edn | .edn | text/x-clojure |
| 1409 | fmt/1604 | EggPaint (Atari Falcon) | trp |  |
| 1410 | linguist/99 | Eiffel | .e | text/x-eiffel |
| 1411 | fmt/1506 | EinScan RGE 3D Range File | rge |  |
| 1412 | fmt/1292 | EIOffice Document | eio |  |
| 1413 | linguist/95 | EJS | .ejs, .ect, .ejs.t, .jst |  |
| 1414 | fmt/1543 | ELAN Annotation File | eaf | application/eaf+xml |
| 1415 | fmt/1544 | ELAN Preference File | pfsx |  |
| 1416 | x-fmt/137 | Electronic Arts Music | asf |  |
| 1417 | fmt/1251 | Electronically Certified Document (EDOC) | edoc | application/vnd.etsi.asic-e+zip |
| 1418 | linguist/100 | Elixir | .ex, .exs |  |
| 1419 | linguist/101 | Elm | .elm | text/x-elm |
| 1420 | linguist/570996448 | Elvish | .elv |  |
| 1421 | linguist/452025714 | Elvish Transcript |  |  |
| 1422 | linguist/102 | Emacs Lisp | .el, .emacs, .emacs.desktop | text/x-common-lisp |
| 1423 | fmt/1382 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| 1424 | fmt/1383 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| 1425 | fmt/1384 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| 1426 | linguist/103 | EmberScript | .em, .emberscript | text/x-coffeescript |
| 1427 | fmt/123 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| 1428 | fmt/124 | Encapsulated PostScript File Format | eps, epsf, ps | application/postscript |
| 1429 | fmt/122 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| 1430 | fmt/1884 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| 1431 | fmt/803 | Encase Image File/Expert Witness Compression File | e01 | application/encase |
| 1432 | fmt/1683 | EndNote Compressed Library | enlx |  |
| 1433 | fmt/1685 | EndNote Compressed Library | enlx |  |
| 1434 | fmt/326 | EndNote Connection File | enz | application/x-endnote-connect, application/x-endnote-connection |
| 1435 | fmt/327 | EndNote Filter File | enf |  |
| 1436 | fmt/328 | EndNote Import File | enw, enr | application/x-endnote-refer |
| 1437 | fmt/325 | EndNote Library | enl |  |
| 1438 | fmt/1684 | EndNote Library | enl |  |
| 1439 | fmt/1682 | EndNote Library | enl |  |
| 1440 | fmt/324 | EndNote Style File | ens | application/x-endnote-style |
| 1441 | fmt/371 | Enhanced Compression Wavelet | ecw |  |
| 1442 | fmt/1856 | Enhanced Image Package | eip |  |
| 1443 | fmt/1971 | Enigma Binary File (Finale) | mus |  |
| 1444 | fmt/397 | Enigma Binary File (Finale) | mus |  |
| 1445 | fmt/1972 | Enigma Binary File (Finale) | mus |  |
| 1446 | fmt/398 | Enigma Transportable File (Finale) | etf |  |
| 1447 | fmt/1580 | Envision Publisher File | evp |  |
| 1448 | fmt/1581 | Envision Publisher Font Files | svf |  |
| 1449 | fmt/1287 | Envoy Document File | evy |  |
| 1450 | fmt/1286 | Envoy Document File | evy |  |
| 1451 | fmt/641 | Epson Raw Image Format | erf |  |
| 1452 | fmt/483 | ePub format | epub | application/epub+zip |
| 1453 | linguist/96 | EQ | .eq | text/x-csharp |
| 1454 | fmt/195 | ERDAS IMAGINE Gray-scale Bitmap Image | gis |  |
| 1455 | fmt/1563 | ERDAS Imagine Large Raster Spill File | ige |  |
| 1456 | linguist/104 | Erlang | .erl, .app, .app.src, .es, .escript, .hrl, .xrl, .yrl | text/x-erlang |
| 1457 | fmt/1369 | Error File | err |  |
| 1458 | fmt/530 | eRuby HTML document | rhtml, rhtm |  |
| 1459 | fmt/1874 | Esko ArtPro File | ap |  |
| 1460 | x-fmt/218 | ESRI Arc/Info Binary Grid | adf |  |
| 1461 | x-fmt/226 | ESRI Arc/Info Export File | e00, x00, e01, e02, e03, e04, e05, e06, e07, e08, e09, e10, e11, e12, e13, e14, e15, e16, e17, e18, e19, e20 |  |
| 1462 | x-fmt/317 | ESRI Arc/View Project | apr |  |
| 1463 | fmt/332 | ESRI Arc/View Project | apr, def |  |
| 1464 | x-fmt/235 | ESRI Arc/View ShapeFile | shp |  |
| 1465 | fmt/277 | ESRI Arc/View Shapefile Index | shx |  |
| 1466 | fmt/1614 | Esri ArcExplorer Project File | aep |  |
| 1467 | fmt/1692 | ESRI ArcGIS Raw Raster Reader/ Writer | hdr |  |
| 1468 | fmt/989 | ESRI ArcGlobe Document | 3dd |  |
| 1469 | fmt/1591 | ESRI ArcInfo Coverage Annotation File | txt |  |
| 1470 | fmt/1594 | ESRI ArcInfo DAT File (External) | dat |  |
| 1471 | fmt/1600 | ESRI ArcInfo DAT File (Internal) |  |  |
| 1472 | fmt/1596 | ESRI ArcInfo Grid .nit File | nit |  |
| 1473 | fmt/916 | ESRI ArcMap Document | mxd, mxt |  |
| 1474 | fmt/1847 | Esri ArcMap Label file | lxp |  |
| 1475 | fmt/988 | ESRI ArcScene Document | sxd |  |
| 1476 | fmt/1696 | ESRI Attribute Index Files | ain |  |
| 1477 | fmt/1253 | ESRI Code Page File | cpg |  |
| 1478 | fmt/1625 | ESRI Colour File Format | clr |  |
| 1479 | fmt/990 | ESRI File Geodatabase |  |  |
| 1480 | x-fmt/225 | ESRI MapInfo Data File | mid |  |
| 1481 | x-fmt/231 | ESRI MapInfo Export File | mif |  |
| 1482 | fmt/1771 | ESRI Persistent Auxiliary Metadata File | xml, aux.xml |  |
| 1483 | fmt/1366 | ESRI Published Map Format | pmf |  |
| 1484 | fmt/1729 | Esri Shapefile Geospatial Metadata File | xml |  |
| 1485 | fmt/321 | ESRI Shapefile Header Index | aih |  |
| 1486 | fmt/320 | ESRI Shapefile Projection (Well-Known Text) Format | prj |  |
| 1487 | fmt/319 | ESRI Spatial Index File | sbn, sbx |  |
| 1488 | fmt/367 | ESRI World File Format | tfw, jgw, pgw, bpw, tifw, blw, bilw, jpgw, rasterw, btw, sdw |  |
| 1489 | fmt/1969 | ETC Express/Expression Show File | shw |  |
| 1490 | linguist/880693982 | Euphoria | .e, .ex |  |
| 1491 | fmt/60 | Excel 95 Workbook (xls) |  |  |
| 1492 | x-fmt/389 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| 1493 | x-fmt/397 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| 1494 | x-fmt/396 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| 1495 | fmt/1507 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| 1496 | fmt/645 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| 1497 | x-fmt/398 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| 1498 | x-fmt/390 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| 1499 | x-fmt/391 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| 1500 | x-fmt/388 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| 1501 | x-fmt/387 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| 1502 | x-fmt/399 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| 1503 | fmt/1090 | Exclude File | exclude |  |
| 1504 | fmt/690 | Executable and Linkable Format | elf, o |  |
| 1505 | fmt/688 | Executable and Linkable Format | elf, o |  |
| 1506 | fmt/689 | Executable and Linkable Format | elf, o |  |
| 1507 | fmt/691 | Executable and Linkable Format | elf, o |  |
| 1508 | fmt/1609 | exFAT (Extensible File Allocation Table) Disc Image | img |  |
| 1509 | fmt/323 | Extended Module Audio File | xm | audio/xm |
| 1510 | fmt/600 | eXtensible ARchive format | xar |  |
| 1511 | fmt/102 | Extensible Hypertext Markup Language | html, htm | application/xhtml+xml |
| 1512 | fmt/103 | Extensible Hypertext Markup Language | htm, html | application/xhtml+xml |
| 1513 | fmt/1776 | Extensible Markup Language | xml | application/xml, text/xml |
| 1514 | fmt/101 | Extensible Markup Language | xml | application/xml, text/xml |
| 1515 | fmt/986 | Extensible Metadata Platform Format | xmp |  |
| 1516 | fmt/570 | Extensible Metadata Platform Packet | xmp |  |
| 1517 | fmt/714 | Extensible Music Format | xmf, mxmf |  |
| 1518 | x-fmt/281 | Extensible Stylesheet Language | xsl | application/xml |
| 1519 | linguist/105 | F# | .fs, .fsi, .fsx | text/x-fsharp |
| 1520 | linguist/336943375 | F* | .fst, .fsti |  |
| 1521 | linguist/108 | Factor | .factor | text/x-factor |
| 1522 | fmt/1353 | FamilyTree Maker Database | ftw, fbk |  |
| 1523 | fmt/1352 | FamilyTree Maker Database | ftw, fbk |  |
| 1524 | linguist/109 | Fancy | .fy, .fancypack |  |
| 1525 | linguist/110 | Fantom | .fan |  |
| 1526 | fmt/723 | Farandole Composer Module | far |  |
| 1527 | fmt/1133 | Farbfeld Image Format | ff |  |
| 1528 | fmt/1397 | FARO Laser Scan File | fls |  |
| 1529 | fmt/1398 | FARO WorkSpace File | fws |  |
| 1530 | fmt/1087 | FAT Disk Image | img, ima, dsk |  |
| 1531 | linguist/622529198 | Faust | .dsp |  |
| 1532 | fmt/1009 | FBX (Filmbox) Binary |  |  |
| 1533 | fmt/1010 | FBX (Filmbox) Text | fbx |  |
| 1534 | fmt/889 | Feather | feather |  |
| 1535 | linguist/239946126 | Fennel | .fnl |  |
| 1536 | linguist/686129783 | FIGlet Font | .flf |  |
| 1537 | linguist/111 | Filebench WML | .f |  |
| 1538 | x-fmt/319 | FileMaker Pro Database | fp5, fmp, fp, fm |  |
| 1539 | fmt/1059 | FileMaker Pro Database | fm |  |
| 1540 | fmt/1072 | FileMaker Pro Database |  |  |
| 1541 | fmt/1237 | FileMaker Pro Database | fmp12 |  |
| 1542 | fmt/194 | FileMaker Pro Database | fp7 |  |
| 1543 | x-fmt/318 | FileMaker Pro Database | fp3, fmp, fp, fm | application/x-filemaker |
| 1544 | linguist/112 | Filterscript | .fs |  |
| 1545 | fmt/1845 | Final Draft Document | fdx |  |
| 1546 | fmt/964 | Final Draft Document | fdr |  |
| 1547 | fmt/1966 | Final Writer Document | fw |  |
| 1548 | fmt/1974 | Finale Notation File | musx | application/vnd.makemusic.notation |
| 1549 | fmt/1973 | Finale Performance Assessment | fpa |  |
| 1550 | fmt/1396 | FinePrint | fp |  |
| 1551 | linguist/906694254 | FIRRTL | .fir |  |
| 1552 | linguist/415 | fish | .fish |  |
| 1553 | x-fmt/110 | Fixed Width Values Text File |  | text/plain |
| 1554 | fmt/733 | FL Studio project file (FLP) | flp |  |
| 1555 | fmt/279 | FLAC (Free Lossless Audio Codec) | flac | audio/flac |
| 1556 | x-fmt/383 | Flexible Image Transport System | fits | application/fits, image/fits |
| 1557 | fmt/1799 | FLExText Interlinear XML Format | flextext |  |
| 1558 | fmt/1410 | Flow Charting | fcx |  |
| 1559 | fmt/1406 | Flow Charting | cht |  |
| 1560 | fmt/1411 | Flow Charting | pdq |  |
| 1561 | fmt/1407 | Flow Charting | fcd |  |
| 1562 | fmt/1408 | Flow Charting | gfc |  |
| 1563 | fmt/1409 | Flow Charting | fc5 |  |
| 1564 | fmt/1412 | Flow Charting Graphic Flowcharting Image | gfi |  |
| 1565 | fmt/1737 | Flow Cytometry Standard File | fcs | application/vnd.isac.fcs |
| 1566 | fmt/1785 | FLR Database File | flr |  |
| 1567 | linguist/206353404 | Fluent | .ftl |  |
| 1568 | linguist/106 | FLUX | .fx, .flux |  |
| 1569 | fmt/1241 | FO File | fo | application/vnd.software602.filler.form+xml |
| 1570 | fmt/1163 | Folio Definition File | def |  |
| 1571 | fmt/1162 | Folio Flat File | fff |  |
| 1572 | fmt/1158 | Folio Infobase File | nfo |  |
| 1573 | fmt/1159 | Folio Infobase File | nfo |  |
| 1574 | fmt/1157 | Folio Infobase File | nfo |  |
| 1575 | fmt/1160 | Folio Shadow File | sdw |  |
| 1576 | fmt/1161 | Folio Shadow File | sdw |  |
| 1577 | apache-httpd/15816773989381630238 | font/collection | ttc | font/collection |
| 1578 | x-fmt/442 | form*Z Project File | fmz | application/octet-stream |
| 1579 | linguist/113 | Formatted | .for, .eam.fs |  |
| 1580 | linguist/114 | Forth | .fth, .4th, .f, .for, .forth, .fr, .frt, .fs | text/x-forth |
| 1581 | linguist/107 | Fortran | .f, .f77, .for, .fpp | text/x-fortran |
| 1582 | fmt/879 | Fortran | f90, f95, f03, f, for |  |
| 1583 | linguist/761352333 | Fortran Free Form | .f90, .f03, .f08, .f95 | text/x-fortran |
| 1584 | fmt/1846 | Fountain Markup Language File | spmd, fountain |  |
| 1585 | fmt/375 | FoxPro Compound Index File | cdx |  |
| 1586 | x-fmt/7 | FoxPro Database | dbf |  |
| 1587 | x-fmt/6 | FoxPro Database | dbf |  |
| 1588 | fmt/373 | FoxPro Database | dbf |  |
| 1589 | fmt/381 | FoxPro Project | pjx |  |
| 1590 | fmt/376 | FoxPro Report | frx |  |
| 1591 | x-fmt/320 | Fractal Image | fif |  |
| 1592 | x-fmt/55 | Frame Vector Metafile | fmv |  |
| 1593 | fmt/1173 | FrameMD5 | framemd5, md5 |  |
| 1594 | x-fmt/321 | Framework Database | fw, fw2 |  |
| 1595 | x-fmt/323 | Framework Database | fw4 |  |
| 1596 | x-fmt/322 | Framework Database | fw3 |  |
| 1597 | fmt/872 | Free Lossless Image Format (FLIF) | flif | image/flif |
| 1598 | fmt/1096 | FreeArc Archive Format | arc |  |
| 1599 | linguist/472896659 | FreeBASIC | .bi, .bas | text/x-vb |
| 1600 | x-fmt/89 | Freelance File | pre |  |
| 1601 | linguist/115 | FreeMarker | .ftl |  |
| 1602 | linguist/116 | Frege | .fr |  |
| 1603 | fmt/642 | Fujifilm RAW Image Format | raf |  |
| 1604 | fmt/1786 | Funpaint Image File | fun, fp2, vic |  |
| 1605 | linguist/97358117 | Futhark | .fut |  |
| 1606 | linguist/117 | G-code | .g, .cnc, .gco, .gcode |  |
| 1607 | fmt/821 | G64 GCR-encoded Disk Image Format | g41, g64, g71 |  |
| 1608 | fmt/1787 | G9B Graphics Format Bitmap | g9b |  |
| 1609 | linguist/125 | Game Maker Language | .gml | text/x-c++src |
| 1610 | linguist/290345951 | GAML | .gaml |  |
| 1611 | linguist/118 | GAMS | .gms |  |
| 1612 | linguist/119 | GAP | .g, .gap, .gd, .gi, .tst |  |
| 1613 | fmt/1651 | Garmin Flexible and Interoperable Data Transfer File | fit |  |
| 1614 | fmt/1679 | Garmin track log file | gmn |  |
| 1615 | fmt/1903 | Garmin Vehicle Images File | srf |  |
| 1616 | fmt/1131 | Gatan Digital Micrograph File Format (DM3) | dm3 |  |
| 1617 | fmt/894 | Gaussian Input Data File | gjf |  |
| 1618 | linguist/121 | GCC Machine Description | .md | text/x-common-lisp |
| 1619 | linguist/122 | GDB | .gdb, .gdbinit |  |
| 1620 | linguist/123 | GDScript | .gd |  |
| 1621 | linguist/459577965 | GEDCOM | .ged |  |
| 1622 | x-fmt/159 | GEM Image | img |  |
| 1623 | x-fmt/215 | GEM Metafile Format | gem |  |
| 1624 | fmt/542 | GEM Metafile Format | gem |  |
| 1625 | fmt/543 | GEM Metafile Format | gem |  |
| 1626 | linguist/907065713 | Gemfile.lock |  |  |
| 1627 | linguist/310828396 | Gemini | .gmi |  |
| 1628 | fmt/1770 | GenBank Flat File | gb, gbk |  |
| 1629 | fmt/851 | Genealogical Data Communication (GEDCOM) Format | ged |  |
| 1630 | fmt/1849 | General Purpose RAW | gpr |  |
| 1631 | x-fmt/425 | Generic Library File | lib |  |
| 1632 | linguist/986054050 | Genero 4gl | .4gl |  |
| 1633 | linguist/902995658 | Genero per | .per |  |
| 1634 | linguist/792408528 | Genie | .gs |  |
| 1635 | linguist/126 | Genshi | .kid | text/xml |
| 1636 | linguist/127 | Gentoo Ebuild | .ebuild | text/x-sh |
| 1637 | linguist/128 | Gentoo Eclass | .eclass | text/x-sh |
| 1638 | fmt/620 | GeoGebra | ggb | application/vnd.geogebra.file |
| 1639 | fmt/618 | GeoGebra | geo | application/vnd.geogebra.file |
| 1640 | fmt/619 | GeoGebra | ggb | application/vnd.geogebra.file |
| 1641 | fmt/622 | GeoGebra | ggb | application/vnd.geogebra.file |
| 1642 | fmt/617 | GeoGebra | ggb | application/vnd.geogebra.file |
| 1643 | fmt/621 | GeoGebra | ggb | application/vnd.geogebra.file |
| 1644 | fmt/155 | Geographic Tagged Image File Format (GeoTIFF) | tif, tiff | image/tiff |
| 1645 | fmt/1047 | Geography Markup Language | gml | application/gml+xml |
| 1646 | x-fmt/227 | Geography Markup Language | gml | application/gml+xml |
| 1647 | fmt/1367 | GeoJSON | geojson | application/geo+json |
| 1648 | fmt/1726 | Geosoft Map Description File | mdf |  |
| 1649 | fmt/664 | Gerber Format | gbr | application/vnd.gerber |
| 1650 | linguist/404627610 | Gerber Image | .gbr, .cmp, .gbl, .gbo, .gbp, .gbs, .gko, .gml, .gpb, .gpt, .gtl, .gto, .gtp, .gts, .ncl, .sol |  |
| 1651 | linguist/129 | Gettext Catalog | .po, .pot |  |
| 1652 | linguist/76 | Gherkin | .feature, .story |  |
| 1653 | fmt/615 | Gimp Image File Format | xcf |  |
| 1654 | linguist/956324166 | Git Attributes |  | text/x-sh |
| 1655 | linguist/807968997 | Git Config | .gitconfig | text/x-properties |
| 1656 | linguist/461881235 | Git Revision List |  |  |
| 1657 | fmt/1316 | GL Transmission Format (Binary) | glb | model/gltf-binary |
| 1658 | fmt/1314 | GL Transmission Format (Text) | gltf | application/json |
| 1659 | fmt/1315 | GL Transmission Format (Text) | gltf | application/json |
| 1660 | linguist/1054258749 | Gleam | .gleam |  |
| 1661 | linguist/5523150 | Glimmer JS | .gjs |  |
| 1662 | linguist/95110458 | Glimmer TS | .gts |  |
| 1663 | linguist/124 | GLSL | .glsl, .fp, .frag, .frg, .fs, .fsh, .fshader, .geo, .geom, .glslf, .glslv, .gs, .gshader, .rchit, .rmiss, .shader, .tesc, .tese, .vert, .vrx, .vs, .vsh, .vshader |  |
| 1664 | linguist/130 | Glyph | .glf | text/x-tcl |
| 1665 | linguist/997665271 | Glyph Bitmap Distribution Format | .bdf |  |
| 1666 | linguist/302957008 | GN | .gn, .gni | text/x-python |
| 1667 | fmt/1844 | GNU Image Manipulation Program Palette File | gpl |  |
| 1668 | fmt/1219 | Gnumeric | gnumeric | application/x-gnumeric |
| 1669 | linguist/131 | Gnuplot | .gp, .gnu, .gnuplot, .p, .plot, .plt |  |
| 1670 | linguist/132 | Go | .go | text/x-go |
| 1671 | linguist/1054391671 | Go Checksums |  |  |
| 1672 | linguist/947461016 | Go Module |  |  |
| 1673 | linguist/934546256 | Go Workspace |  |  |
| 1674 | fmt/1834 | GoDot 4Bit Graphics Format | 4bt |  |
| 1675 | linguist/738107771 | Godot Resource | .gdnlib, .gdns, .tres, .tscn |  |
| 1676 | linguist/133 | Golo | .golo |  |
| 1677 | fmt/1073 | Google Document Link File | gslides, gdoc, gsheet, gdraw, gmap, gsite, gform |  |
| 1678 | linguist/134 | Gosu | .gs, .gst, .gsx, .vark |  |
| 1679 | fmt/1134 | GPS Exchange Format | gpx |  |
| 1680 | fmt/243 | GPS Exchange Format | gpx |  |
| 1681 | linguist/135 | Grace | .grace |  |
| 1682 | linguist/136 | Gradle | .gradle |  |
| 1683 | linguist/432600901 | Gradle Kotlin DSL | .gradle.kts |  |
| 1684 | linguist/137 | Grammatical Framework | .gf | text/x-haskell |
| 1685 | linguist/138 | Graph Modeling Language | .gml |  |
| 1686 | fmt/336 | Graphic Workshop for Windows Thumbnail File | thn |  |
| 1687 | fmt/4 | Graphics Interchange Format | gif | image/gif |
| 1688 | fmt/3 | Graphics Interchange Format | gif | image/gif |
| 1689 | fmt/1913 | Graphisoft Archicad Project | pln, pla |  |
| 1690 | fmt/1955 | Graphisoft Archicad Project | pla, pln |  |
| 1691 | fmt/1914 | Graphisoft BIMx Hyper-Model | bimx |  |
| 1692 | fmt/575 | GraphPad Prism | pzm |  |
| 1693 | fmt/576 | GraphPad Prism | pzf |  |
| 1694 | linguist/139 | GraphQL | .graphql, .gql, .graphqls |  |
| 1695 | linguist/140 | Graphviz (DOT) | .dot, .gv |  |
| 1696 | fmt/285 | Gridded Binary | grb, wmo |  |
| 1697 | fmt/284 | Gridded Binary | grb, wmo |  |
| 1698 | linguist/142 | Groovy | .groovy, .grt, .gtpl, .gvy | text/x-groovy |
| 1699 | linguist/143 | Groovy Server Pages | .gsp | application/x-jsp |
| 1700 | linguist/257856279 | GSC | .gsc, .csc, .gsh | text/x-csrc |
| 1701 | fmt/362 | GSSI SIR-10 RADAN data file | dzt |  |
| 1702 | fmt/1878 | GST Art File | art |  |
| 1703 | fmt/1877 | GST Art File | art |  |
| 1704 | fmt/1415 | GST Publisher File | dtp |  |
| 1705 | fmt/1416 | GST Publisher File | dtp |  |
| 1706 | fmt/1872 | Guitar Pro File | gtp |  |
| 1707 | fmt/1873 | Guitar Pro File | gp3, gp4, gp5 |  |
| 1708 | fmt/1788 | Gunpaint Image File | gun |  |
| 1709 | fmt/1202 | Guymager Acquisition Info File | info |  |
| 1710 | fmt/1789 | GX2 Graphics File | gx2, ega |  |
| 1711 | x-fmt/266 | GZIP Format | gz, z | application/gzip |
| 1712 | linguist/153 | Hack | .hack, .hh, .hhi, .php | application/x-httpd-php |
| 1713 | fmt/1791 | Haiku Vector Icon Format | hvif |  |
| 1714 | linguist/154 | Haml | .haml, .haml.deface | text/x-haml |
| 1715 | linguist/155 | Handlebars | .handlebars, .hbs |  |
| 1716 | fmt/1083 | Hangul Word Processor Document | hwp |  |
| 1717 | fmt/1084 | Hangul Word Processor Document | hwp |  |
| 1718 | linguist/366607477 | HAProxy | .cfg |  |
| 1719 | linguist/156 | Harbour | .hb |  |
| 1720 | linguist/463518941 | Hare | .ha |  |
| 1721 | fmt/426 | Harris Matrix | hm |  |
| 1722 | x-fmt/32 | Harvard Graphics Chart | ch3 |  |
| 1723 | fmt/1492 | Harvard Graphics Presentation | pr4 |  |
| 1724 | fmt/1491 | Harvard Graphics Presentation | prs |  |
| 1725 | x-fmt/101 | Harvard Graphics Show | sh3 |  |
| 1726 | x-fmt/324 | Harvard Graphics Show | shw |  |
| 1727 | x-fmt/325 | Harvard Graphics Vector Graphics | cht |  |
| 1728 | linguist/157 | Haskell | .hs, .hs-boot, .hsc | text/x-haskell |
| 1729 | fmt/1062 | Hasselblad 3FR Raw Image | 3fr |  |
| 1730 | linguist/158 | Haxe | .hx, .hxsl | text/x-haxe |
| 1731 | linguist/144 | HCL | .hcl, .nomad, .tf, .tfvars, .workflow | text/x-ruby |
| 1732 | fmt/1041 | HDF | hdf, h4 |  |
| 1733 | fmt/287 | HDF5 | hdf5, h5, hdf, nc |  |
| 1734 | fmt/807 | HDF5 | h5, hdf, hdf5, nc |  |
| 1735 | fmt/286 | HDF5 | hdf, h5, hdf5, nc |  |
| 1736 | fmt/1790 | Help Librarian File | hlp, dat, dta |  |
| 1737 | x-fmt/326 | Hewlett Packard AdvanceWrite Text File | aw |  |
| 1738 | x-fmt/293 | Hewlett Packard Graphics Language | hpgl | application/vnd.hp-HPGL |
| 1739 | fmt/1174 | Hewlett Packard Graphics Language | 000 | application/vnd.hp-HPGL |
| 1740 | x-fmt/83 | Hewlett Packard Vector Graphic Plotter File | plt |  |
| 1741 | fmt/1105 | Hierarchical File System | img |  |
| 1742 | fmt/1742 | Hierarchical File System Plus | img, dmg, toast |  |
| 1743 | fmt/1101 | High Efficiency Image File Format | heic | image/heif |
| 1744 | linguist/931814087 | HiveQL | .q, .hql |  |
| 1745 | linguist/145 | HLSL | .hlsl, .cginc, .fx, .fxh, .hlsli |  |
| 1746 | fmt/1876 | HMM Packfile | pak |  |
| 1747 | linguist/679725279 | HOCON | .hocon |  |
| 1748 | linguist/928121743 | HolyC | .hc | text/x-csrc |
| 1749 | linguist/560883276 | hoon | .hoon |  |
| 1750 | linguist/231021894 | Hosts File |  |  |
| 1751 | fmt/1332 | HP Photo Album | albm |  |
| 1752 | fmt/1212 | HP System Software Manager CVA File | cva |  |
| 1753 | fmt/1423 | HP TRIM Outlook Saved Message File | vmbx, mbx |  |
| 1754 | linguist/146 | HTML | .html, .hta, .htm, .html.hl, .inc, .xht, .xhtml | text/html |
| 1755 | fmt/886 | HTML Components | htc |  |
| 1756 | x-fmt/417 | HTML Extension File | htx |  |
| 1757 | linguist/148 | HTML+ECR | .ecr | text/html |
| 1758 | linguist/149 | HTML+EEX | .eex, .html.heex, .html.leex | text/html |
| 1759 | linguist/150 | HTML+ERB | .erb, .erb.deface, .rhtml | application/x-erb |
| 1760 | linguist/151 | HTML+PHP | .phtml | application/x-httpd-php |
| 1761 | linguist/479039817 | HTML+Razor | .cshtml, .razor | text/html |
| 1762 | linguist/152 | HTTP | .http | message/http |
| 1763 | fmt/1843 | Human Machine Interfaces HMI File | hmi |  |
| 1764 | fmt/2001 | Husqvarna / Pfaff Embroidery Stitch File | vip |  |
| 1765 | fmt/2003 | Husqvarna / Premier+ Embroidery Stitch File | vp4 |  |
| 1766 | fmt/2002 | Husqvarna / TruE Embroidery Stitch File | vp3 |  |
| 1767 | fmt/2000 | Husqvarna Embroidery Stitch File | hus |  |
| 1768 | fmt/2004 | Husqvarna-Viking Designer 1 Stitch File | shv, mhv, phv |  |
| 1769 | linguist/786683730 | HXML | .hxml |  |
| 1770 | linguist/159 | Hy | .hy |  |
| 1771 | fmt/1490 | HyperCard Stack |  |  |
| 1772 | fmt/96 | Hypertext Markup Language | htm, html | text/html |
| 1773 | fmt/100 | Hypertext Markup Language | htm, html | text/html |
| 1774 | fmt/98 | Hypertext Markup Language | htm, html | text/html |
| 1775 | fmt/471 | Hypertext Markup Language | htm, html | text/html |
| 1776 | fmt/97 | Hypertext Markup Language | htm, html | text/html |
| 1777 | fmt/99 | Hypertext Markup Language | htm, html | text/html |
| 1778 | linguist/160 | HyPhy | .bf |  |
| 1779 | fmt/893 | i2 Analysts Notebook | anb |  |
| 1780 | x-fmt/148 | IBM DisplayWrite DCA Text File | dca |  |
| 1781 | x-fmt/289 | IBM DisplayWrite Document |  |  |
| 1782 | x-fmt/288 | IBM DisplayWrite Document |  |  |
| 1783 | x-fmt/284 | IBM DisplayWrite Final Form Text File | fft |  |
| 1784 | x-fmt/285 | IBM DisplayWrite Revisable Form Text File | rft |  |
| 1785 | linguist/98384424 | iCalendar | .ics, .ical | text/x-properties |
| 1786 | fmt/1975 | ICC Profile | icc, icm | application/vnd.iccprofile |
| 1787 | fmt/1977 | ICC Profile | icc, icm | application/vnd.iccprofile |
| 1788 | fmt/1976 | ICC Profile | icc, icm | application/vnd.iccprofile |
| 1789 | fmt/1793 | ICDRAW Group Icon File | ib3 |  |
| 1790 | fmt/1792 | ICDRAW Single Icon File | ibi |  |
| 1791 | fmt/1400 | Ichitaro Document | jtd, jtt, $td | application/x-js-taro |
| 1792 | x-fmt/418 | Icon file format | ico | image/vnd.microsoft.icon, image/x-icon |
| 1793 | linguist/161 | IDL | .pro, .dlm | text/x-idl |
| 1794 | linguist/165 | Idris | .idr, .lidr |  |
| 1795 | fmt/1288 | IESNA LM-63 Photometric Data File | ies |  |
| 1796 | linguist/74444240 | Ignore List | .gitignore | text/x-sh |
| 1797 | linguist/162 | IGOR Pro | .ipf |  |
| 1798 | fmt/578 | Image Cytometry Standard | ics |  |
| 1799 | fmt/577 | Image Cytometry Standard | ics |  |
| 1800 | apache-httpd/14184589139419150626 | image/avif | avif | image/avif |
| 1801 | apache-httpd/5785170104885681682 | image/g3fax | g3 | image/g3fax |
| 1802 | apache-httpd/11916099667387573904 | image/ief | ief | image/ief |
| 1803 | apache-httpd/3404436964465585248 | image/prs.btif | btif | image/prs.btif |
| 1804 | apache-httpd/10059028400364481609 | image/sgi | sgi | image/sgi |
| 1805 | apache-httpd/1112684080820345302 | image/vnd.dece.graphic | uvi, uvvi, uvg, uvvg | image/vnd.dece.graphic |
| 1806 | apache-httpd/2327811077702530899 | image/vnd.dvb.subtitle | sub | image/vnd.dvb.subtitle |
| 1807 | apache-httpd/6089710766004459381 | image/vnd.fastbidsheet | fbs | image/vnd.fastbidsheet |
| 1808 | apache-httpd/17531001443158399273 | image/vnd.fst | fst | image/vnd.fst |
| 1809 | apache-httpd/12828564092157514067 | image/vnd.fujixerox.edmics-mmr | mmr | image/vnd.fujixerox.edmics-mmr |
| 1810 | apache-httpd/675748516876487325 | image/vnd.fujixerox.edmics-rlc | rlc | image/vnd.fujixerox.edmics-rlc |
| 1811 | apache-httpd/12869456523622353063 | image/vnd.ms-photo | wdp | image/vnd.ms-photo |
| 1812 | apache-httpd/6068593002644094061 | image/vnd.net-fpx | npx | image/vnd.net-fpx |
| 1813 | apache-httpd/16355469457160075364 | image/vnd.wap.wbmp | wbmp | image/vnd.wap.wbmp |
| 1814 | apache-httpd/12470378540611442998 | image/x-3ds | 3ds | image/x-3ds |
| 1815 | apache-httpd/12815764328499810916 | image/x-cmu-raster | ras | image/x-cmu-raster |
| 1816 | apache-httpd/3283330744365083168 | image/x-cmx | cmx | image/x-cmx |
| 1817 | apache-httpd/12759051345378655462 | image/x-freehand | fh, fhc, fh4, fh5, fh7 | image/x-freehand |
| 1818 | apache-httpd/10459519546450867951 | image/x-mrsid-image | sid | image/x-mrsid-image |
| 1819 | apache-httpd/1383629243372609493 | image/x-pcx | pcx | image/x-pcx |
| 1820 | apache-httpd/10418899209645427276 | image/x-portable-anymap | pnm | image/x-portable-anymap |
| 1821 | apache-httpd/3427479898683859874 | image/x-portable-graymap | pgm | image/x-portable-graymap |
| 1822 | apache-httpd/9581195091460280608 | image/x-rgb | rgb | image/x-rgb |
| 1823 | apache-httpd/6911424546062404041 | image/x-tga | tga | image/x-tga |
| 1824 | linguist/575143428 | ImageJ Macro | .ijm |  |
| 1825 | linguist/1057618448 | Imba | .imba |  |
| 1826 | fmt/1997 | IMF Package Asset Map | xml |  |
| 1827 | fmt/1999 | IMF Package Composition Playlist | xml |  |
| 1828 | fmt/1998 | IMF Package Packing List | xml |  |
| 1829 | fmt/982 | iMovieProj File Format | iMovieProj |  |
| 1830 | fmt/1206 | Impulse 3D Data Description Object | iob |  |
| 1831 | fmt/715 | Impulse Tracker Module | it |  |
| 1832 | fmt/1184 | InDesign Markup Language Package | idml | application/vnd.adobe.indesign-idml-package |
| 1833 | fmt/700 | Industry Foundation Classes | ifc |  |
| 1834 | fmt/699 | Industry Foundation Classes | ifc |  |
| 1835 | fmt/659 | Industry Foundation Classes | ifc |  |
| 1836 | fmt/663 | Industry Foundation Classes XML | ifcXML |  |
| 1837 | linguist/166 | Inform 7 | .ni, .i7x |  |
| 1838 | fmt/212 | Information or Setup File | inf |  |
| 1839 | linguist/163 | INI | .ini, .cfg, .cnf, .dof, .lektorproject, .prefs, .pro, .properties, .url | text/x-properties |
| 1840 | x-fmt/158 | Initial Graphics Exchange Specification (IGES) | iges, igs | model/iges |
| 1841 | linguist/838252715 | Ink | .ink |  |
| 1842 | x-fmt/95 | Inkwriter/Notetaker Document | pwi |  |
| 1843 | x-fmt/81 | Inkwriter/Notetaker Template | pdt |  |
| 1844 | linguist/167 | Inno Setup | .iss, .isl |  |
| 1845 | x-fmt/171 | Inset Systems Bitmap | pix |  |
| 1846 | fmt/1647 | Inspiration Software File | isf |  |
| 1847 | x-fmt/180 | Instalit Script | pvd |  |
| 1848 | fmt/197 | InstallShield Compiled Rules File | inx |  |
| 1849 | fmt/1466 | InstallShield Executable | ex_ |  |
| 1850 | x-fmt/327 | IntelliDraw Vector Graphics | idw |  |
| 1851 | fmt/1675 | IntelliFont Font File | type, lib |  |
| 1852 | x-fmt/328 | InterBase Database | gdb |  |
| 1853 | x-fmt/157 | Interchange File | iff |  |
| 1854 | fmt/339 | Interchange File Format 8-bit Sampled Voice | iff, 8svx |  |
| 1855 | fmt/338 | Interchange File Format Interleaved Bitmap | iff, lbm |  |
| 1856 | x-fmt/229 | Intergraph Raster Image | ing |  |
| 1857 | x-fmt/329 | Interleaf Document | doc |  |
| 1858 | fmt/841 | Interleaved ADX Audio Format (AIX) | aix |  |
| 1859 | fmt/1014 | INTERLIS Model File | ili |  |
| 1860 | fmt/654 | INTERLIS Model File | ili |  |
| 1861 | fmt/1012 | INTERLIS Model File | ili |  |
| 1862 | fmt/653 | INTERLIS Transfer File | xtf |  |
| 1863 | fmt/1013 | INTERLIS Transfer File | itf |  |
| 1864 | fmt/1011 | INTERLIS Transfer File | xml, xtf |  |
| 1865 | x-fmt/219 | Internet Archive | arc | application/x-internet-archive |
| 1866 | fmt/410 | Internet Archive | arc |  |
| 1867 | fmt/388 | Internet Calendar and Scheduling format | ics | text/calendar |
| 1868 | fmt/358 | Internet Data Query File | idq |  |
| 1869 | fmt/500 | Internet Explorer for Mac cache file | waf |  |
| 1870 | fmt/278 | Internet Message Format | eml | message/rfc822 |
| 1871 | fmt/1680 | INTREPID Standard Information File | isi |  |
| 1872 | linguist/168 | Io | .io |  |
| 1873 | linguist/169 | Ioke | .ik |  |
| 1874 | linguist/164 | IRC log | .irclog, .weechatlog | text/mirc |
| 1875 | x-fmt/194 | IRIS Graphics |  |  |
| 1876 | linguist/170 | Isabelle | .thy |  |
| 1877 | linguist/171 | Isabelle ROOT |  |  |
| 1878 | fmt/1570 | ISDOC Information System Document | isdoc |  |
| 1879 | fmt/1567 | ISDOC Information System Document | isdoc |  |
| 1880 | fmt/1571 | ISDOCX Information System Document | isdocx |  |
| 1881 | fmt/1568 | ISDOCX Information System Document | isdocx |  |
| 1882 | fmt/468 | ISO 9660 Disk Image File | iso, toast, cdr, dmg, bin |  |
| 1883 | linguist/172 | J | .ijs |  |
| 1884 | fmt/975 | Jamcracker Tracker Module | jam |  |
| 1885 | linguist/1028705371 | Janet | .janet | text/x-scheme |
| 1886 | linguist/447261135 | JAR Manifest |  |  |
| 1887 | fmt/1125 | JASCO JWS Format | jws |  |
| 1888 | linguist/180 | Jasmin | .j |  |
| 1889 | linguist/181 | Java | .java, .jav, .jsh | text/x-java |
| 1890 | x-fmt/412 | Java Archive Format | jar | application/java-archive |
| 1891 | x-fmt/415 | Java Class File | class |  |
| 1892 | x-fmt/422 | Java Language Source Code File | java |  |
| 1893 | linguist/519377561 | Java Properties | .properties | text/x-properties |
| 1894 | linguist/182 | Java Server Pages | .jsp, .tag | application/x-jsp |
| 1895 | x-fmt/160 | Java Servlet Page | jsp | text/html |
| 1896 | linguist/599494012 | Java Template Engine | .jte |  |
| 1897 | linguist/183 | JavaScript | .js, ._js, .bones, .cjs, .es, .es6, .frag, .gs, .jake, .javascript, .jsb, .jscad, .jsfl, .jslib, .jsm, .jspre, .jss, .jsx, .mjs, .njs, .pac, .sjs, .ssjs, .xsjs, .xsjslib | text/javascript |
| 1898 | x-fmt/423 | JavaScript file | js | application/javascript |
| 1899 | linguist/914318960 | JavaScript+ERB | .js.erb | application/javascript |
| 1900 | linguist/316620079 | JCL | .jcl |  |
| 1901 | fmt/994 | Jeffs Image Format | jif |  |
| 1902 | fmt/895 | JEOL NMR Spectroscopy | jdf |  |
| 1903 | linguist/774635084 | Jest Snapshot | .snap | application/javascript |
| 1904 | linguist/465165328 | JetBrains MPS | .mps, .mpl, .msd | text/xml |
| 1905 | linguist/173 | JFlex | .flex, .jflex |  |
| 1906 | linguist/147 | Jinja | .jinja, .j2, .jinja2 | text/x-django |
| 1907 | linguist/284531423 | Jison | .jison |  |
| 1908 | linguist/406395330 | Jison Lex | .jisonlex |  |
| 1909 | linguist/998078858 | Jolie | .ol, .iol |  |
| 1910 | x-fmt/392 | JP2 (JPEG 2000 part 1) | jp2 | image/jp2 |
| 1911 | fmt/1794 | JPEG 2000 Codestream | j2k, jpc, j2c | image/jp2 |
| 1912 | fmt/590 | JPEG Extended Range | wdp, jxr | image/jxr |
| 1913 | fmt/43 | JPEG File Interchange Format | jpg, jpe, jpeg, jif, jfif, jfi | image/jpeg |
| 1914 | fmt/44 | JPEG File Interchange Format | jpg, jpe, jpeg, jif, jfif, jfi | image/jpeg |
| 1915 | fmt/42 | JPEG File Interchange Format | jpeg, jpe, jpg, jif, jfif, jfi | image/jpeg |
| 1916 | fmt/529 | JPEG Network Graphics | jng | image/x-jng |
| 1917 | fmt/1485 | JPEG XL | jxl | image/jxl |
| 1918 | fmt/1484 | JPEG XL Codestream | jxl | image/jxl |
| 1919 | fmt/150 | JPEG-LS | jls |  |
| 1920 | fmt/1964 | JPH (JPEG 2000 part 15) | jph | image/jph |
| 1921 | fmt/463 | JPM (JPEG 2000 part 6) | jpm | image/jpm |
| 1922 | fmt/151 | JPX (JPEG 2000 part 2) | jpx, jpf | image/jpx |
| 1923 | linguist/905371884 | jq | .jq |  |
| 1924 | linguist/174 | JSON | .json, .4DForm, .4DProject, .avsc, .geojson, .gltf, .har, .ice, .JSON-tmLanguage, .json.example, .jsonl, .mcmeta, .sarif, .tact, .tfstate, .tfstate.backup, .topojson, .webapp, .webmanifest, .yy, .yyp | application/json |
| 1925 | fmt/817 | JSON Data Interchange Format | json | application/json |
| 1926 | linguist/423 | JSON with Comments | .jsonc, .code-snippets, .code-workspace, .sublime-build, .sublime-color-scheme, .sublime-commands, .sublime-completions, .sublime-keymap, .sublime-macro, .sublime-menu, .sublime-mousemap, .sublime-project, .sublime-settings, .sublime-theme, .sublime-workspace, .sublime_metrics, .sublime_session | text/javascript |
| 1927 | fmt/880 | JSON-LD | jsonld |  |
| 1928 | linguist/175 | JSON5 | .json5 | application/json |
| 1929 | linguist/177 | JSONiq | .jq | application/json |
| 1930 | linguist/176 | JSONLD | .jsonld | application/json |
| 1931 | linguist/664885656 | Jsonnet | .jsonnet, .libsonnet |  |
| 1932 | fmt/149 | JTIP (JPEG Tiled Image Pyramid) |  |  |
| 1933 | linguist/184 | Julia | .jl | text/x-julia |
| 1934 | linguist/220689142 | Julia REPL |  |  |
| 1935 | fmt/1908 | Jupiter Tesselation (JT) File | jt |  |
| 1936 | linguist/185 | Jupyter Notebook | .ipynb | application/json |
| 1937 | fmt/1119 | Jupyter Python Notebook | ipynb |  |
| 1938 | linguist/128447695 | Just | .just |  |
| 1939 | x-fmt/330 | JustWrite Text Document | jw, jwt |  |
| 1940 | linguist/818804755 | Kaitai Struct | .ksy | text/x-yaml |
| 1941 | linguist/603336474 | KakouneScript | .kak |  |
| 1942 | linguist/59716426 | KerboScript | .ks |  |
| 1943 | fmt/724 | Keyhole Markup Language (Container) | kmz | application/vnd.google-earth.kmz |
| 1944 | fmt/244 | Keyhole Markup Language (XML) | kml | application/vnd.google-earth.kml+xml |
| 1945 | fmt/970 | Khronos Texture File | ktx | image/ktx |
| 1946 | linguist/187 | KiCad Layout | .kicad_pcb, .kicad_mod, .kicad_wks | text/x-common-lisp |
| 1947 | linguist/140848857 | KiCad Legacy Layout | .brd |  |
| 1948 | linguist/622447435 | KiCad Schematic | .kicad_sch, .sch |  |
| 1949 | linguist/692635484 | Kickstart | .ks |  |
| 1950 | linguist/188 | Kit | .kit | text/html |
| 1951 | fmt/1780 | Koala MicroIllustrator Graphic File | pic |  |
| 1952 | fmt/192 | Kodak Digital Camera Raw Image File | dcr |  |
| 1953 | x-fmt/56 | Kodak FlashPix Image | fpx | image/vnd.fpx |
| 1954 | fmt/211 | Kodak Photo CD Image | pcd |  |
| 1955 | x-fmt/165 | Kodak PhotoCD Image |  |  |
| 1956 | linguist/189 | Kotlin | .kt, .ktm, .kts | text/x-kotlin |
| 1957 | fmt/999 | Krita Document Format | kra | application/x-krita |
| 1958 | linguist/186 | KRL | .krl |  |
| 1959 | fmt/656 | KryoFlux | raw |  |
| 1960 | fmt/655 | KryoFlux | raw |  |
| 1961 | linguist/225697190 | Kusto | .csl, .kql |  |
| 1962 | linguist/970675279 | kvlang | .kv |  |
| 1963 | linguist/194 | LabVIEW | .lvproj, .lvclass, .lvlib | text/xml |
| 1964 | linguist/758480799 | Lark | .lark | text/x-ebnf |
| 1965 | linguist/195 | Lasso | .lasso, .las, .lasso8, .lasso9 |  |
| 1966 | fmt/280 | LaTeX (Master document) |  |  |
| 1967 | fmt/281 | LaTeX (Subdocument) |  |  |
| 1968 | linguist/196 | Latte | .latte | text/x-smarty |
| 1969 | fmt/611 | LDAP Data Interchange Format | ldif |  |
| 1970 | fmt/1336 | LEADTools Lead 1Bit Compressed Image | cmp |  |
| 1971 | fmt/1337 | LEADToolsCompressed Image | cmp |  |
| 1972 | fmt/1063 | Leaf Mosaic Raw Image | mos |  |
| 1973 | linguist/197 | Lean | .lean, .hlean |  |
| 1974 | linguist/455147478 | Lean 4 | .lean |  |
| 1975 | fmt/1868 | Leapfrog Geo 3D Scene Format | lfsc |  |
| 1976 | fmt/1345 | Legacy Family Tree Database | fdb |  |
| 1977 | fmt/1724 | LegalDocML Document | xml |  |
| 1978 | fmt/1838 | Leica Project File | lgs |  |
| 1979 | fmt/1643 | Lenel Network Video Recorder File | lnr |  |
| 1980 | fmt/1217 | Leonardo Image Format | leo |  |
| 1981 | linguist/198 | Less | .less | text/css |
| 1982 | linguist/199 | Lex | .l, .lex |  |
| 1983 | linguist/190 | LFE | .lfe | text/x-common-lisp |
| 1984 | fmt/626 | LHA File Format | lha, lzh |  |
| 1985 | x-fmt/426 | License file | lic |  |
| 1986 | fmt/587 | LifeTechnologies ABIF | abif |  |
| 1987 | fmt/586 | LifeTechnologies SDS | sds |  |
| 1988 | fmt/1205 | LightWave 3D Object | lw |  |
| 1989 | fmt/1153 | Lightwright Show File | lw3 |  |
| 1990 | fmt/1151 | Lightwright Show File | lw1, lw |  |
| 1991 | fmt/1156 | Lightwright Show File | lw6 |  |
| 1992 | fmt/1152 | Lightwright Show File | lw2 |  |
| 1993 | fmt/1154 | Lightwright Show File | lw4 |  |
| 1994 | fmt/1155 | Lightwright Show File | lw5 |  |
| 1995 | linguist/1040646257 | LigoLANG | .ligo | text/x-pascal |
| 1996 | linguist/200 | LilyPond | .ly, .ily |  |
| 1997 | linguist/201 | Limbo | .b, .m |  |
| 1998 | linguist/202 | Linker Script | .ld, .lds, .x |  |
| 1999 | linguist/203 | Linux Kernel Module | .mod |  |
| 2000 | fmt/1672 | Linux/i386 Binary Executable File ZMAGIC | so, o |  |
| 2001 | linguist/204 | Liquid | .liquid |  |
| 2002 | linguist/205 | Literate Agda | .lagda |  |
| 2003 | linguist/206 | Literate CoffeeScript | .litcoffee, .coffee.md |  |
| 2004 | linguist/207 | Literate Haskell | .lhs | text/x-literate-haskell |
| 2005 | linguist/891017 | LiveCode Script | .livecodescript |  |
| 2006 | fmt/1921 | LiveCode Stack | rev, livecode |  |
| 2007 | fmt/1922 | LiveCode Stack |  |  |
| 2008 | fmt/1923 | LiveCode Stack | rev, livecode |  |
| 2009 | fmt/1920 | LiveCode Stack | rev, livecode |  |
| 2010 | linguist/208 | LiveScript | .ls, ._ls | text/x-livescript |
| 2011 | linguist/191 | LLVM | .ll |  |
| 2012 | fmt/1310 | LocoFile |  |  |
| 2013 | fmt/1305 | LocoScript Document |  |  |
| 2014 | fmt/1304 | LocoScript Document |  |  |
| 2015 | fmt/1306 | LocoScript Document |  |  |
| 2016 | fmt/1307 | LocoScript Document |  |  |
| 2017 | fmt/1308 | LocoScript PC |  |  |
| 2018 | fmt/1309 | LocoScript Professional |  |  |
| 2019 | fmt/391 | Log ASCII Standard Format | las |  |
| 2020 | fmt/390 | Log ASCII Standard Format | las |  |
| 2021 | fmt/389 | Log ASCII Standard Format | las |  |
| 2022 | x-fmt/62 | Log File | log |  |
| 2023 | fmt/804 | Logical File Evidence Format | l01 |  |
| 2024 | linguist/209 | Logos | .xm, .x, .xi |  |
| 2025 | linguist/210 | Logtalk | .lgt, .logtalk |  |
| 2026 | linguist/192 | LOLCODE | .lol |  |
| 2027 | linguist/211 | LookML | .lkml, .lookml | text/x-yaml |
| 2028 | linguist/212 | LoomScript | .ls |  |
| 2029 | x-fmt/82 | Lotus 1-2-3 Chart | pic |  |
| 2030 | x-fmt/331 | Lotus 1-2-3 Spreadsheet Formatting File | fm1, fmt |  |
| 2031 | x-fmt/332 | Lotus 1-2-3 Spreadsheet Formatting File | fm3 |  |
| 2032 | x-fmt/116 | Lotus 1-2-3 Worksheet | wk4 | application/lotus123, application/vnd.lotus-1-2-3 |
| 2033 | fmt/1452 | Lotus 1-2-3 Worksheet | 123 | application/vnd.lotus-1-2-3, application/x-123 |
| 2034 | x-fmt/212 | Lotus 1-2-3 Worksheet |  |  |
| 2035 | x-fmt/115 | Lotus 1-2-3 Worksheet | wk3 | application/lotus123, application/vnd.lotus-1-2-3 |
| 2036 | x-fmt/117 | Lotus 1-2-3 Worksheet | wks | application/vnd.lotus-1-2-3, application/x-123 |
| 2037 | x-fmt/114 | Lotus 1-2-3 Worksheet | wk1, wk2 | application/vnd.lotus-1-2-3, application/x-123 |
| 2038 | fmt/1453 | Lotus 1-2-3 Worksheet | 123 | application/vnd.lotus-1-2-3, application/x-123 |
| 2039 | x-fmt/333 | Lotus Approach View File | apr | application/vnd.lotus-approach |
| 2040 | x-fmt/334 | Lotus Approach View File | apt | application/vnd.lotus-approach |
| 2041 | fmt/1216 | Lotus Freelance Show | prz | application/vnd.lotus-freelance |
| 2042 | x-fmt/335 | Lotus Freelance Smartmaster Graphics | mas | application/vnd.lotus-freelance |
| 2043 | x-fmt/337 | Lotus Notes Database | ns3, nsf | application/vnd.lotus-notes |
| 2044 | x-fmt/338 | Lotus Notes Database | ns4, nsf | application/vnd.lotus-notes |
| 2045 | x-fmt/336 | Lotus Notes Database | ns2, nsf | application/vnd.lotus-notes |
| 2046 | x-fmt/339 | Lotus Notes File | box |  |
| 2047 | fmt/1938 | Lotus Screencam Data File | scm | application/vnd.lotus-screencam |
| 2048 | fmt/340 | Lotus WordPro Document | lwp | application/lwp, application/vnd.lotus-wordpro |
| 2049 | x-fmt/340 | Lotus WordPro Document | lwp | application/lwp, application/vnd.lotus-wordpro |
| 2050 | linguist/193 | LSL | .lsl, .lslp |  |
| 2051 | linguist/1013566805 | LTspice Symbol | .asy | text/x-spreadsheet |
| 2052 | linguist/213 | Lua | .lua, .fcgi, .nse, .p8, .pd_lua, .rbxs, .rockspec, .wlua | text/x-lua |
| 2053 | linguist/365050359 | Luau | .luau | text/x-lua |
| 2054 | linguist/214 | M | .mumps, .m | text/x-mumps |
| 2055 | fmt/1055 | M2TS | mts, m2ts |  |
| 2056 | linguist/215 | M4 | .m4, .mc |  |
| 2057 | linguist/216 | M4Sugar | .m4 |  |
| 2058 | linguist/34167825 | Macaulay2 | .m2 |  |
| 2059 | fmt/1762 | MacBinary | bin |  |
| 2060 | fmt/1763 | MacBinary | bin |  |
| 2061 | fmt/1761 | MacBinary |  |  |
| 2062 | fmt/1819 | MacCaption File | mcc |  |
| 2063 | fmt/1820 | MacCaption File | mcc |  |
| 2064 | fmt/1821 | MacCaption Project | cca |  |
| 2065 | fmt/1426 | MacDraw |  |  |
| 2066 | fmt/1425 | MacDraw |  |  |
| 2067 | fmt/1427 | MacDraw |  |  |
| 2068 | fmt/1428 | MacDraw |  |  |
| 2069 | fmt/693 | Mach-O |  |  |
| 2070 | fmt/692 | Mach-O |  |  |
| 2071 | x-fmt/80 | Macintosh PICT Image | pct, pict | image/x-pict |
| 2072 | fmt/341 | Macintosh PICT Image | pct, pict, pic | image/x-pict |
| 2073 | x-fmt/14 | Macintosh Text File |  | text/plain |
| 2074 | x-fmt/175 | MacPaint Graphics | pnt |  |
| 2075 | x-fmt/161 | MacPaint Image | mac |  |
| 2076 | fmt/1429 | MacPaint Image |  |  |
| 2077 | fmt/487 | Macro Enabled Microsoft Powerpoint | pptm | application/vnd.ms-powerpoint.presentation.macroEnabled.12 |
| 2078 | fmt/523 | Macro enabled Microsoft Word Document OOXML | docm | application/vnd.ms-word.document.macroEnabled.12 |
| 2079 | fmt/486 | Macromedia (Adobe) Director Compressed Resource file | dcr |  |
| 2080 | x-fmt/341 | Macromedia Director | dir, dxr | application/x-director |
| 2081 | fmt/317 | Macromedia Director | dir, dxr | application/x-director |
| 2082 | fmt/105 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2083 | fmt/104 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2084 | fmt/108 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2085 | fmt/109 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2086 | fmt/107 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2087 | fmt/106 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2088 | fmt/110 | Macromedia Flash | swf | application/x-shockwave-flash |
| 2089 | x-fmt/382 | Macromedia FLV | flv | video/x-flv |
| 2090 | x-fmt/53 | Macromedia Freehand | fh5, fh4 |  |
| 2091 | fmt/544 | Macromedia FreeHand | fh7 |  |
| 2092 | fmt/546 | Macromedia FreeHand | fh9 |  |
| 2093 | fmt/545 | Macromedia FreeHand | fh8 |  |
| 2094 | fmt/547 | Macromedia FreeHand | fh10 |  |
| 2095 | fmt/400 | Macromedia FreeHand MX | fh11 |  |
| 2096 | fmt/1464 | Maestro Music File |  |  |
| 2097 | fmt/1472 | Magic Shadow Archiver Disk Image File | msa | application/vnd.msa-disk-image |
| 2098 | fmt/976 | MagicaVoxel Vox format | vox |  |
| 2099 | fmt/930 | Magick Image File Format | mif |  |
| 2100 | linguist/220 | Makefile | .mak, .d, .make, .makefile, .mk, .mkfile | text/x-cmake |
| 2101 | fmt/1469 | MAKIchan Graphics File | mki, mag, max |  |
| 2102 | linguist/221 | Mako | .mako, .mao |  |
| 2103 | x-fmt/221 | MapBrowser/MapWriter Vector Map Data | cbd |  |
| 2104 | fmt/1256 | MapInfo Workspace File | wor |  |
| 2105 | fmt/915 | Mapsforge Binary Map File Format | map |  |
| 2106 | fmt/1875 | Maptech BSB Documentation File | bsb, kap |  |
| 2107 | fmt/1483 | Mar Archive | mar, mac |  |
| 2108 | fmt/1149 | Markdown | md, markdown | text/markdown |
| 2109 | linguist/222 | Markdown | .md, .livemd, .markdown, .mdown, .mdwn, .mkd, .mkdn, .mkdown, .ronn, .scd, .workbook | text/x-gfm |
| 2110 | linguist/932782397 | Marko | .marko | text/html |
| 2111 | linguist/223 | Mask | .mask |  |
| 2112 | fmt/1900 | Mass Spectrometry Markup Language | mxml |  |
| 2113 | fmt/789 | Material Exchange Format | mxf | application/mxf |
| 2114 | fmt/785 | Material Exchange Format | mxf | application/mxf |
| 2115 | fmt/786 | Material Exchange Format | mxf | application/mxf |
| 2116 | fmt/787 | Material Exchange Format | mxf | application/mxf |
| 2117 | fmt/790 | Material Exchange Format | mxf | application/mxf |
| 2118 | fmt/791 | Material Exchange Format | mxf | application/mxf |
| 2119 | fmt/784 | Material Exchange Format | mxf | application/mxf |
| 2120 | fmt/788 | Material Exchange Format | mxf | application/mxf |
| 2121 | fmt/200 | Material Exchange Format | mxf | application/mxf |
| 2122 | fmt/783 | Material Exchange Format | mxf | application/mxf |
| 2123 | fmt/931 | Mathcad Document | mcd |  |
| 2124 | fmt/932 | Mathcad Document | xmcd |  |
| 2125 | linguist/224 | Mathematica | .mathematica, .cdf, .m, .ma, .mt, .nb, .nbp, .wl, .wlt | text/x-mathematica |
| 2126 | fmt/201 | Mathematica Notebook | nb | application/mathematica |
| 2127 | linguist/225 | MATLAB | .matlab, .m | text/x-octave |
| 2128 | fmt/1550 | MATLAB Mat File | mat |  |
| 2129 | fmt/806 | MATLAB Mat File | mat, fig |  |
| 2130 | fmt/828 | MATLAB Mat File | mat, fig |  |
| 2131 | fmt/1678 | MATLAB Script File | m |  |
| 2132 | fmt/569 | Matroska | mkv, mk3d, mka, mks |  |
| 2133 | linguist/226 | Maven POM |  | text/xml |
| 2134 | linguist/227 | Max | .maxpat, .maxhelp, .maxproj, .mxt, .pat | application/json |
| 2135 | linguist/217 | MAXScript | .ms, .mcr |  |
| 2136 | fmt/1146 | Maxwell Render Image Format | mxi |  |
| 2137 | fmt/1145 | Maxwell Render Material File | mxm |  |
| 2138 | fmt/1147 | Maxwell Render Scene File Format | mxs |  |
| 2139 | fmt/863 | Maya ASCII File Format | ma |  |
| 2140 | fmt/861 | Maya Binary File Format | mb |  |
| 2141 | fmt/862 | Maya Binary File Format | mb |  |
| 2142 | fmt/1168 | Maya Icons or Swatches file | icons, swatches |  |
| 2143 | fmt/1169 | Maya IFF Image File | iff, ico |  |
| 2144 | fmt/720 | MBOX | mbox | application/mbox |
| 2145 | linguist/462488745 | mcfunction | .mcfunction |  |
| 2146 | fmt/993 | MD5 File | md5 |  |
| 2147 | linguist/512838272 | MDX | .mdx | text/x-gfm |
| 2148 | fmt/1758 | Media Descriptor File | mdf |  |
| 2149 | fmt/1759 | Media Descriptor Sidecar File | mds |  |
| 2150 | fmt/1765 | Media Hash List | mhl |  |
| 2151 | fmt/648 | Media View Pro | mpcatalog |  |
| 2152 | fmt/1959 | Melco OFM Project | ofm |  |
| 2153 | fmt/1958 | Melco OFM Project | ofm |  |
| 2154 | fmt/1890 | Memory Stick Voice File (MSV) | msv |  |
| 2155 | fmt/1892 | Memory Stick Voice File (MSV)/Digital Voice File (DVF) | msv, dvf |  |
| 2156 | linguist/229 | Mercury | .m, .moo |  |
| 2157 | linguist/385992043 | Mermaid | .mmd, .mermaid |  |
| 2158 | linguist/799141244 | Meson |  |  |
| 2159 | fmt/1918 | MetaCard Stack | mc, rev |  |
| 2160 | linguist/230 | Metal | .metal | text/x-c++src |
| 2161 | x-fmt/429 | MHTML | mht, mhtml | multipart/related |
| 2162 | x-fmt/296 | Micrografx Designer | drw |  |
| 2163 | x-fmt/151 | Micrografx Designer | dsf |  |
| 2164 | x-fmt/295 | Micrografx Draw | drw, drt |  |
| 2165 | x-fmt/294 | Micrografx Draw | drw |  |
| 2166 | x-fmt/47 | Micrografx Draw | drw |  |
| 2167 | fmt/1907 | Micrografx Icon File | icn |  |
| 2168 | fmt/1481 | Micrografx In-A-Vision Drawing | pic |  |
| 2169 | fmt/275 | Microsoft Access Database File | accdb |  |
| 2170 | fmt/1806 | Microsoft Access Database File | mdb, mda |  |
| 2171 | x-fmt/239 | Microsoft Access Database File | mdb, mda, mdt, mde |  |
| 2172 | x-fmt/66 | Microsoft Access Database File | mdb, mda |  |
| 2173 | x-fmt/241 | Microsoft Access Database File | mdb, mde |  |
| 2174 | x-fmt/238 | Microsoft Access Database File | mdb, mda, mde, mdt |  |
| 2175 | fmt/1805 | Microsoft Access Database File | mdb, mda |  |
| 2176 | x-fmt/240 | Microsoft Access Database File | mdb, mde |  |
| 2177 | fmt/1809 | Microsoft Access Encrypted Database File | mdb, mda |  |
| 2178 | fmt/1807 | Microsoft Access Encrypted Database File | mdb, mda |  |
| 2179 | fmt/1808 | Microsoft Access Encrypted Database File | mdb, mda |  |
| 2180 | fmt/1258 | Microsoft Access Workgroup Information File | mdw |  |
| 2181 | fmt/1893 | Microsoft Agent File | acs |  |
| 2182 | fmt/386 | Microsoft Animated Cursor Format | ani |  |
| 2183 | fmt/634 | Microsoft Compiled HTML Help | chm, chw | application/vnd.ms-htmlhelp |
| 2184 | linguist/800983837 | Microsoft Developer Studio Project | .dsp |  |
| 2185 | fmt/881 | Microsoft Document Imaging File Format | mdi | image/vnd.ms-modi |
| 2186 | fmt/55 | Microsoft Excel 2.x Worksheet (xls) | xls | application/vnd.ms-excel |
| 2187 | fmt/62 | Microsoft Excel 2000-2003 Workbook (xls) | xlw, xls | application/vnd.ms-excel |
| 2188 | fmt/56 | Microsoft Excel 3.0 Worksheet (xls) | xls | application/vnd.ms-excel |
| 2189 | fmt/58 | Microsoft Excel 4.0 Workbook (xls) | xlw | application/vnd.ms-excel |
| 2190 | fmt/57 | Microsoft Excel 4.0 Worksheet (xls) | xls | application/vnd.ms-excel |
| 2191 | fmt/59 | Microsoft Excel 5.0/95 Workbook (xls) | xlw, xls | application/vnd.ms-excel |
| 2192 | fmt/61 | Microsoft Excel 97 Workbook (xls) | xls, xlw | application/vnd.ms-excel |
| 2193 | x-fmt/124 | Microsoft Excel Add-In | xla, xll |  |
| 2194 | x-fmt/23 | Microsoft Excel Backup | xlk |  |
| 2195 | x-fmt/126 | Microsoft Excel Chart | xlc |  |
| 2196 | fmt/554 | Microsoft Excel Chart | xlc | application/vnd.ms-excel |
| 2197 | fmt/553 | Microsoft Excel Chart | xlc | application/vnd.ms-excel |
| 2198 | fmt/174 | Microsoft Excel for Macintosh |  |  |
| 2199 | fmt/178 | Microsoft Excel for Macintosh |  |  |
| 2200 | fmt/173 | Microsoft Excel for Macintosh |  |  |
| 2201 | fmt/176 | Microsoft Excel for Macintosh |  |  |
| 2202 | fmt/172 | Microsoft Excel for Macintosh |  |  |
| 2203 | fmt/177 | Microsoft Excel for Macintosh |  |  |
| 2204 | fmt/175 | Microsoft Excel for Macintosh |  |  |
| 2205 | fmt/214 | Microsoft Excel for Windows | xlsx | application/vnd.openxmlformats-officedocument.spreadsheetml.sheet |
| 2206 | x-fmt/123 | Microsoft Excel Macro | xla, xlm | application/vnd.ms-excel |
| 2207 | fmt/555 | Microsoft Excel Macro | xlm | application/vnd.ms-excel |
| 2208 | fmt/556 | Microsoft Excel Macro | xlm | application/vnd.ms-excel |
| 2209 | fmt/445 | Microsoft Excel Macro-Enabled | xlsm | application/vnd.ms-excel.sheet.macroEnabled.12 |
| 2210 | fmt/628 | Microsoft Excel Macro-Enabled Add-In | xlam | application/vnd.ms-excel.addin.macroEnabled.12 |
| 2211 | fmt/627 | Microsoft Excel Macro-Enabled Template | xltm | application/vnd.ms-excel.template.macroEnabled.12 |
| 2212 | fmt/595 | Microsoft Excel Non-XML Binary Workbook | xlsb | application/vnd.ms-excel.sheet.binary.macroEnabled.12 |
| 2213 | x-fmt/46 | Microsoft Excel ODBC Query | dqy |  |
| 2214 | x-fmt/74 | Microsoft Excel OLAP Query | oqy |  |
| 2215 | x-fmt/97 | Microsoft Excel OLE DB Query | rqy |  |
| 2216 | x-fmt/17 | Microsoft Excel Template | xlt | application/vnd.ms-excel |
| 2217 | fmt/598 | Microsoft Excel Template | xltx | application/vnd.openxmlformats-officedocument.spreadsheetml.template |
| 2218 | x-fmt/125 | Microsoft Excel Toolbar | xlb |  |
| 2219 | x-fmt/58 | Microsoft Excel Web Query | iqy |  |
| 2220 | fmt/1858 | Microsoft Excel Workspace File | xlw | application/vnd.ms-excel |
| 2221 | x-fmt/128 | Microsoft Excel Workspace File | xlw |  |
| 2222 | fmt/647 | Microsoft Expression Media | ivc |  |
| 2223 | x-fmt/242 | Microsoft FoxPro Database | dbf |  |
| 2224 | x-fmt/172 | Microsoft FoxPro Library | plb |  |
| 2225 | x-fmt/342 | Microsoft FoxPro Memo | fpt, frt, vct, pjt |  |
| 2226 | fmt/359 | Microsoft Front Page Binary Tree Index | btr |  |
| 2227 | fmt/288 | Microsoft Front Page Server Extension Configuration |  |  |
| 2228 | fmt/218 | Microsoft FrontPage | lck |  |
| 2229 | fmt/1656 | Microsoft Help Contents File | cnt |  |
| 2230 | x-fmt/454 | Microsoft Internet Shortcut | url | text/plain |
| 2231 | fmt/475 | Microsoft Management Console Snap-in Control file | msc |  |
| 2232 | fmt/1362 | Microsoft MapPoint Document | ptm |  |
| 2233 | fmt/162 | Microsoft Multiplan | mod |  |
| 2234 | fmt/778 | Microsoft Network Monitor Packet Capture | cap |  |
| 2235 | fmt/777 | Microsoft Network Monitor Packet Capture | cap |  |
| 2236 | fmt/240 | Microsoft Office Binder File for Windows | obd |  |
| 2237 | fmt/237 | Microsoft Office Binder File for Windows | obd |  |
| 2238 | fmt/241 | Microsoft Office Binder Template for Windows | obt |  |
| 2239 | fmt/238 | Microsoft Office Binder Template for Windows | obt |  |
| 2240 | fmt/242 | Microsoft Office Binder Wizard for Windows | obz |  |
| 2241 | fmt/239 | Microsoft Office Binder Wizard for Windows | obz |  |
| 2242 | fmt/494 | Microsoft Office Encrypted Document | xlsx, pptx, docx |  |
| 2243 | fmt/1677 | Microsoft Office File List | xml |  |
| 2244 | fmt/189 | Microsoft Office Open XML |  |  |
| 2245 | fmt/473 | Microsoft Office Owner File | doc, docx |  |
| 2246 | fmt/524 | Microsoft Office Theme | thmx | application/vnd.ms-officetheme |
| 2247 | fmt/637 | Microsoft OneNote | one | application/msonenote |
| 2248 | fmt/987 | Microsoft OneNote Package File | onepkg |  |
| 2249 | x-fmt/73 | Microsoft Outlook Address Book | olk |  |
| 2250 | x-fmt/430 | Microsoft Outlook Email Message | msg, oft |  |
| 2251 | x-fmt/75 | Microsoft Outlook Personal Address Book | pab |  |
| 2252 | x-fmt/250 | Microsoft Outlook Personal Folders |  |  |
| 2253 | x-fmt/251 | Microsoft Outlook Personal Folders |  |  |
| 2254 | x-fmt/248 | Microsoft Outlook Personal Folders (ANSI) | pst |  |
| 2255 | x-fmt/249 | Microsoft Outlook Personal Folders (Unicode) | pst |  |
| 2256 | x-fmt/214 | Microsoft Paint | msp |  |
| 2257 | fmt/912 | Microsoft Paint | msp |  |
| 2258 | fmt/594 | Microsoft PhotoDraw | mix | image/vnd.mix |
| 2259 | fmt/936 | Microsoft Picture It! Image File | mix | image/vnd.mix |
| 2260 | x-fmt/86 | Microsoft Powerpoint Add-In | ppa |  |
| 2261 | x-fmt/84 | Microsoft Powerpoint Design Template | pot |  |
| 2262 | fmt/1866 | Microsoft Powerpoint for Macintosh | ppt | application/vnd.ms-PowerPoint |
| 2263 | fmt/180 | Microsoft PowerPoint for Macintosh |  |  |
| 2264 | fmt/181 | Microsoft PowerPoint for Macintosh | ppt | application/vnd.ms-powerpoint |
| 2265 | fmt/179 | Microsoft PowerPoint for Macintosh | ppt | application/vnd.ms-powerpoint |
| 2266 | fmt/182 | Microsoft PowerPoint for Macintosh |  |  |
| 2267 | fmt/1867 | Microsoft Powerpoint for Macintosh | ppt | application/vnd.ms-PowerPoint |
| 2268 | fmt/215 | Microsoft Powerpoint for Windows | pptx | application/vnd.openxmlformats-officedocument.presentationml.presentation |
| 2269 | x-fmt/177 | Microsoft PowerPoint Graphics File | ppi |  |
| 2270 | fmt/633 | Microsoft PowerPoint Macro-Enabled Add-In | ppam | application/vnd.ms-powerpoint.addin.macroEnabled.12 |
| 2271 | fmt/630 | Microsoft PowerPoint Macro-Enabled Show | ppsm | application/vnd.ms-powerpoint.slideshow.macroEnabled.12 |
| 2272 | fmt/636 | Microsoft PowerPoint Macro-Enabled Slide | sldm | application/vnd.ms-powerpoint.slide.macroEnabled.12 |
| 2273 | fmt/632 | Microsoft PowerPoint Macro-Enabled Template | potm | application/vnd.ms-powerpoint.template.macroEnabled.12 |
| 2274 | x-fmt/216 | Microsoft Powerpoint Packaged Presentation | ppz |  |
| 2275 | fmt/1747 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-PowerPoint |
| 2276 | fmt/126 | Microsoft Powerpoint Presentation | ppt | application/vnd.ms-powerpoint |
| 2277 | x-fmt/88 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-powerpoint |
| 2278 | fmt/125 | Microsoft Powerpoint Presentation | ppt | application/vnd.ms-powerpoint |
| 2279 | fmt/1748 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-PowerPoint |
| 2280 | x-fmt/87 | Microsoft Powerpoint Presentation Show | pps | application/vnd.ms-powerpoint |
| 2281 | fmt/629 | Microsoft PowerPoint Show | ppsx | application/vnd.openxmlformats-officedocument.presentationml.slideshow |
| 2282 | fmt/631 | Microsoft PowerPoint Template | potx | application/vnd.openxmlformats-officedocument.presentationml.template |
| 2283 | x-fmt/90 | Microsoft Print File | prn |  |
| 2284 | fmt/1078 | Microsoft Program Database | pdb |  |
| 2285 | fmt/1079 | Microsoft Program Database | pdb |  |
| 2286 | x-fmt/245 | Microsoft Project | mpp | application/vnd.ms-project |
| 2287 | x-fmt/247 | Microsoft Project | mpp | application/vnd.ms-project |
| 2288 | x-fmt/243 | Microsoft Project | mpp | application/vnd.ms-project |
| 2289 | fmt/440 | Microsoft Project | mpp | application/vnd.ms-project |
| 2290 | x-fmt/246 | Microsoft Project |  |  |
| 2291 | fmt/725 | Microsoft Project | mpp | application/vnd.ms-project |
| 2292 | x-fmt/244 | Microsoft Project | mpp | application/vnd.ms-project |
| 2293 | fmt/343 | Microsoft Project Export File | mpx | application/x-project |
| 2294 | x-fmt/232 | Microsoft Project Export File | mpx | application/x-project |
| 2295 | fmt/342 | Microsoft Project Export File | mpx | application/x-project |
| 2296 | fmt/1043 | Microsoft PRX File | prx |  |
| 2297 | x-fmt/252 | Microsoft Publisher | pub | application/x-mspublisher |
| 2298 | fmt/1516 | Microsoft Publisher | pub | application/x-mspublisher |
| 2299 | fmt/1513 | Microsoft Publisher | pub | application/x-mspublisher |
| 2300 | x-fmt/256 | Microsoft Publisher | pub | application/x-mspublisher |
| 2301 | x-fmt/255 | Microsoft Publisher | pub | application/x-mspublisher |
| 2302 | fmt/1514 | Microsoft Publisher | pub | application/x-mspublisher |
| 2303 | x-fmt/257 | Microsoft Publisher | pub | application/x-mspublisher |
| 2304 | fmt/1511 | Microsoft Publisher | pub | application/x-mspublisher |
| 2305 | fmt/1515 | Microsoft Publisher | pub | application/x-mspublisher |
| 2306 | x-fmt/254 | Microsoft Publisher | pub | application/x-mspublisher |
| 2307 | fmt/1512 | Microsoft Publisher | pub | application/x-mspublisher |
| 2308 | x-fmt/253 | Microsoft Publisher | pub | application/x-mspublisher |
| 2309 | fmt/1839 | Microsoft Publisher Packaged Document | puz |  |
| 2310 | fmt/867 | Microsoft Reader eBook | lit |  |
| 2311 | fmt/1303 | Microsoft Shell Scrap Object File | shs |  |
| 2312 | x-fmt/106 | Microsoft Symbolic Link (SYLK) File | slk |  |
| 2313 | fmt/442 | Microsoft Visio (generic) | vsd | application/vnd.visio |
| 2314 | x-fmt/113 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| 2315 | fmt/924 | Microsoft Visio Drawing | vsdx | application/vnd.ms-visio.drawing.main+xml |
| 2316 | fmt/1510 | Microsoft Visio Drawing | vsd, vst, vss, vsw | application/vnd.visio |
| 2317 | fmt/1508 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| 2318 | fmt/443 | Microsoft Visio Drawing | vsd | application/vnd.visio |
| 2319 | fmt/1509 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| 2320 | x-fmt/258 | Microsoft Visio Drawing | vsd, vss, vst | application/vnd.visio |
| 2321 | x-fmt/259 | Microsoft Visio Drawing |  |  |
| 2322 | fmt/927 | Microsoft Visio Macro-Enabled Drawing | vsdm | application/vnd.ms-visio.drawing.macroEnabled.main+xml |
| 2323 | fmt/928 | Microsoft Visio Macro-Enabled Stencil | vssm | application/vnd.ms-visio.stencil.macroEnabled.main+xml |
| 2324 | fmt/929 | Microsoft Visio Macro-Enabled Template | vstm | application/vnd.ms-visio.template.macroEnabled.main+xml |
| 2325 | fmt/925 | Microsoft Visio Stencil | vssx | application/vnd.ms-visio.stencil.main+xml |
| 2326 | fmt/926 | Microsoft Visio Template | vstx | application/vnd.ms-visio.template.main+xml |
| 2327 | fmt/216 | Microsoft Visio XML Drawing | vdx | application/vnd.visio |
| 2328 | fmt/379 | Microsoft Visual FoxPro Class Library | vcx |  |
| 2329 | fmt/384 | Microsoft Visual FoxPro database container (memo files) | dct |  |
| 2330 | fmt/382 | Microsoft Visual FoxPro database container (table files) | dbc |  |
| 2331 | fmt/374 | Microsoft Visual FoxPro Database Table File | dbf |  |
| 2332 | fmt/380 | Microsoft Visual FoxPro Project | pjx |  |
| 2333 | fmt/377 | Microsoft Visual FoxPro Report | frx |  |
| 2334 | x-fmt/343 | Microsoft Visual FoxPro Table | dbx |  |
| 2335 | x-fmt/179 | Microsoft Visual Modeller Petal file (ASCII) | ptl |  |
| 2336 | linguist/849523096 | Microsoft Visual Studio Solution | .sln |  |
| 2337 | fmt/385 | Microsoft Windows Cursor | cur | image/x-win-bitmap |
| 2338 | fmt/345 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| 2339 | x-fmt/153 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| 2340 | fmt/344 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| 2341 | fmt/971 | Microsoft Windows Movie Maker File | mswmm |  |
| 2342 | x-fmt/428 | Microsoft Windows Shortcut | lnk |  |
| 2343 | fmt/609 | Microsoft Word (Generic) | doc | application/msword |
| 2344 | fmt/39 | Microsoft Word Document | doc | application/msword |
| 2345 | fmt/40 | Microsoft Word Document | doc, wbk | application/msword |
| 2346 | fmt/754 | Microsoft Word Document (Password Protected) | wbk, doc | application/msword |
| 2347 | x-fmt/45 | Microsoft Word Document Template | dot |  |
| 2348 | fmt/755 | Microsoft Word Document Template (Password Protected) | dot | application/msword |
| 2349 | x-fmt/129 | Microsoft Word for Macintosh Document |  |  |
| 2350 | x-fmt/64 | Microsoft Word for Macintosh Document | mcw | application/msword |
| 2351 | fmt/346 | Microsoft Word for Macintosh Document | mcw | application/msword |
| 2352 | x-fmt/65 | Microsoft Word for Macintosh Document | mcw | application/msword |
| 2353 | x-fmt/2 | Microsoft Word for Macintosh Document |  |  |
| 2354 | x-fmt/1 | Microsoft Word for Macintosh Document | mcw | application/msword |
| 2355 | x-fmt/274 | Microsoft Word for MS-DOS Document | doc | application/msword |
| 2356 | fmt/1688 | Microsoft Word for MS-DOS Document | doc | application/msword |
| 2357 | x-fmt/276 | Microsoft Word for MS-DOS Document | doc | application/msword |
| 2358 | x-fmt/275 | Microsoft Word for MS-DOS Document | doc | application/msword |
| 2359 | x-fmt/273 | Microsoft Word for MS-DOS Document |  |  |
| 2360 | fmt/1689 | Microsoft Word for MS-DOS Glossary File | gly | application/msword |
| 2361 | fmt/1691 | Microsoft Word for MS-DOS Printer Description File | prd | application/msword |
| 2362 | fmt/1690 | Microsoft Word for MS-DOS Style Sheet File | sty | application/msword |
| 2363 | fmt/412 | Microsoft Word for Windows | docx, wbk | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| 2364 | fmt/38 | Microsoft Word for Windows Document | doc | application/msword |
| 2365 | fmt/37 | Microsoft Word for Windows Document | doc | application/msword |
| 2366 | x-fmt/204 | Microsoft Word for Windows Macro | wpm |  |
| 2367 | fmt/599 | Microsoft Word Macro-Enabled Document Template | dotm | application/vnd.ms-word.template.macroEnabled.12 |
| 2368 | fmt/597 | Microsoft Word Template | dotx | application/vnd.openxmlformats-officedocument.wordprocessingml.template |
| 2369 | x-fmt/344 | Microsoft Works Database | bdb |  |
| 2370 | fmt/260 | Microsoft Works Database for DOS | wdb |  |
| 2371 | fmt/169 | Microsoft Works Database for DOS | wdb |  |
| 2372 | fmt/261 | Microsoft Works Database for DOS | wdb |  |
| 2373 | fmt/170 | Microsoft Works Database for DOS | wdb |  |
| 2374 | fmt/171 | Microsoft Works Database for DOS | wdb |  |
| 2375 | fmt/259 | Microsoft Works Database for DOS | wdb |  |
| 2376 | fmt/268 | Microsoft Works Database for Macintosh | wdb |  |
| 2377 | fmt/269 | Microsoft Works Database for Macintosh | wdb |  |
| 2378 | fmt/223 | Microsoft Works Database for Windows | wdb |  |
| 2379 | fmt/252 | Microsoft Works Database for Windows | wdb |  |
| 2380 | fmt/256 | Microsoft Works Database for Windows | wdb |  |
| 2381 | fmt/249 | Microsoft Works Database for Windows | wdb |  |
| 2382 | fmt/225 | Microsoft Works Database for Windows | wdb |  |
| 2383 | fmt/222 | Microsoft Works Database for Windows | wdb |  |
| 2384 | fmt/224 | Microsoft Works Database for Windows | wdb |  |
| 2385 | fmt/226 | Microsoft Works Database for Windows | wdb |  |
| 2386 | fmt/246 | Microsoft Works Database for Windows | wdb |  |
| 2387 | fmt/219 | Microsoft Works Database for Windows | wdb |  |
| 2388 | x-fmt/345 | Microsoft Works Document | bps |  |
| 2389 | x-fmt/120 | Microsoft Works for Windows |  |  |
| 2390 | fmt/901 | Microsoft Works Spreadsheet | xlr |  |
| 2391 | x-fmt/118 | Microsoft Works Spreadsheet |  |  |
| 2392 | fmt/166 | Microsoft Works Spreadsheet | wks |  |
| 2393 | fmt/262 | Microsoft Works Spreadsheet for DOS |  |  |
| 2394 | fmt/168 | Microsoft Works Spreadsheet for DOS |  |  |
| 2395 | fmt/264 | Microsoft Works Spreadsheet for DOS |  |  |
| 2396 | fmt/263 | Microsoft Works Spreadsheet for DOS |  |  |
| 2397 | fmt/167 | Microsoft Works Spreadsheet for DOS |  |  |
| 2398 | fmt/271 | Microsoft Works Spreadsheet for Macintosh | wks |  |
| 2399 | fmt/270 | Microsoft Works Spreadsheet for Macintosh | wks |  |
| 2400 | fmt/230 | Microsoft Works Spreadsheet for Windows |  |  |
| 2401 | fmt/253 | Microsoft Works Spreadsheet for Windows |  |  |
| 2402 | fmt/247 | Microsoft Works Spreadsheet for Windows |  |  |
| 2403 | fmt/220 | Microsoft Works Spreadsheet for Windows |  |  |
| 2404 | fmt/231 | Microsoft Works Spreadsheet for Windows |  |  |
| 2405 | fmt/228 | Microsoft Works Spreadsheet for Windows |  |  |
| 2406 | fmt/229 | Microsoft Works Spreadsheet for Windows |  |  |
| 2407 | fmt/227 | Microsoft Works Spreadsheet for Windows |  |  |
| 2408 | fmt/257 | Microsoft Works Spreadsheet for Windows |  |  |
| 2409 | fmt/250 | Microsoft Works Spreadsheet for Windows |  |  |
| 2410 | fmt/163 | Microsoft Works Word Processor 1-3 for DOS and 2 for Windows | wps |  |
| 2411 | fmt/233 | Microsoft Works Word Processor 3-4 for Windows | wps |  |
| 2412 | fmt/258 | Microsoft Works Word Processor 5-6 | wps |  |
| 2413 | fmt/267 | Microsoft Works Word Processor DOS |  |  |
| 2414 | fmt/265 | Microsoft Works Word Processor DOS |  |  |
| 2415 | fmt/266 | Microsoft Works Word Processor DOS |  |  |
| 2416 | fmt/165 | Microsoft Works Word Processor for DOS |  |  |
| 2417 | fmt/164 | Microsoft Works Word Processor for DOS |  |  |
| 2418 | fmt/232 | Microsoft Works Word Processor for Windows |  |  |
| 2419 | fmt/221 | Microsoft Works Word Processor for Windows |  |  |
| 2420 | fmt/235 | Microsoft Works Word Processor for Windows |  |  |
| 2421 | fmt/236 | Microsoft Works Word Processor for Windows |  |  |
| 2422 | fmt/234 | Microsoft Works Word Processor for Windows |  |  |
| 2423 | fmt/273 | Microsoft Works Word Processor Macintosh | wps |  |
| 2424 | fmt/272 | Microsoft Works Word Processor Macintosh | wps |  |
| 2425 | fmt/248 | Microsoft Works Word Processor Windows |  |  |
| 2426 | fmt/254 | Microsoft Works Word Processor Windows |  |  |
| 2427 | fmt/251 | Microsoft Works Word Processor Windows |  |  |
| 2428 | fmt/923 | Microsoft xWMA | xwma |  |
| 2429 | fmt/1358 | MicroStation Base File | bse |  |
| 2430 | x-fmt/346 | Microstation CAD Drawing | dgn |  |
| 2431 | fmt/1177 | MicroStation Material Library | mat |  |
| 2432 | fmt/1183 | MicroStation Material Palette | pal |  |
| 2433 | fmt/1626 | MicroStation Symbology Resource File | rsc |  |
| 2434 | x-fmt/230 | MIDI Audio | mid, midi | audio/midi |
| 2435 | fmt/1470 | MIG Graphics File | mig |  |
| 2436 | fmt/950 | MIME Email | eml | message/rfc822 |
| 2437 | fmt/1136 | MiniCAD | mcd | application/vnd.vectorworks |
| 2438 | fmt/1137 | MiniCAD | mcd | application/vnd.vectorworks |
| 2439 | fmt/1138 | MiniCAD/VectorWorks | mcd, vwx | application/vnd.vectorworks |
| 2440 | linguist/231 | MiniD | .minid |  |
| 2441 | fmt/1431 | Minitab Portable Worksheet | mtp |  |
| 2442 | fmt/1436 | Minitab Project | mpj |  |
| 2443 | fmt/1434 | Minitab Project | mpj |  |
| 2444 | fmt/1438 | Minitab Project | mpj |  |
| 2445 | fmt/1435 | Minitab Worksheet | mtw |  |
| 2446 | fmt/1432 | Minitab Worksheet | mtw |  |
| 2447 | fmt/1437 | Minitab Worksheet | mtw |  |
| 2448 | fmt/1433 | Minitab Worksheet | mtw |  |
| 2449 | fmt/1430 | Minitab Worksheet | mtw |  |
| 2450 | linguist/4896465 | MiniYAML | .yaml, .yml | text/x-yaml |
| 2451 | fmt/669 | Minolta RAW | mrw |  |
| 2452 | linguist/968740319 | Mint | .mint |  |
| 2453 | linguist/232 | Mirah | .druby, .duby, .mirah | text/x-ruby |
| 2454 | linguist/517654727 | mIRC Script | .mrc |  |
| 2455 | fmt/337 | MJ2 (Motion JPEG 2000) | mj2, mjp2 | video/mj2 |
| 2456 | linguist/448253929 | MLIR | .mlir |  |
| 2457 | fmt/961 | Mobile eXtensible Music Format | mxmf | audio/mobile-xmf |
| 2458 | fmt/716 | MOD Audio Module | mod |  |
| 2459 | apache-httpd/6674260812445752722 | model/mesh | msh, mesh, silo | model/mesh |
| 2460 | apache-httpd/13969526719176485485 | model/vnd.gdl | gdl | model/vnd.gdl |
| 2461 | apache-httpd/18176455083221241937 | model/vnd.gtw | gtw | model/vnd.gtw |
| 2462 | apache-httpd/9317680701060824198 | model/vnd.vtu | vtu | model/vnd.vtu |
| 2463 | apache-httpd/6898497964742600926 | model/x3d+binary | x3db, x3dbz | model/x3d+binary |
| 2464 | apache-httpd/13507959258524992275 | model/x3d+vrml | x3dv, x3dvz | model/x3d+vrml |
| 2465 | apache-httpd/2323420538982214786 | model/x3d+xml | x3d, x3dz | model/x3d+xml |
| 2466 | linguist/233 | Modelica | .mo | text/x-modelica |
| 2467 | linguist/234 | Modula-2 | .mod |  |
| 2468 | linguist/564743864 | Modula-3 | .i3, .ig, .m3, .mg |  |
| 2469 | linguist/235 | Module Management System | .mms, .mmk |  |
| 2470 | linguist/1045019587 | Mojo | .mojo | text/x-python |
| 2471 | linguist/236 | Monkey | .monkey, .monkey2 |  |
| 2472 | linguist/231751931 | Monkey C | .mc | text/x-csrc |
| 2473 | fmt/1086 | Monkey's Audio File | ape |  |
| 2474 | linguist/237 | Moocode | .moo |  |
| 2475 | linguist/181453007 | MoonBit | .mbt |  |
| 2476 | linguist/238 | MoonScript | .moon |  |
| 2477 | fmt/612 | Mork | mab, msf, dat |  |
| 2478 | linguist/202937027 | Motoko | .mo |  |
| 2479 | linguist/477582706 | Motorola 68K Assembly | .asm, .i, .inc, .s, .x68 |  |
| 2480 | linguist/638334599 | Move | .move |  |
| 2481 | fmt/1970 | MOXCEL | mxl |  |
| 2482 | fmt/134 | MPEG 1/2 Audio Layer 3 | mp3 | audio/mpeg |
| 2483 | x-fmt/279 | MPEG 1/2 Audio Layer 3 Streaming | m3u | audio/mpeg |
| 2484 | fmt/347 | MPEG 1/2 Audio Layer I | mp1 | audio/mpeg |
| 2485 | fmt/198 | MPEG Audio Stream Layer II | mp2, mpw, mpa | audio/mpeg |
| 2486 | fmt/649 | MPEG-1 Elementary Stream | mpg, mpeg, m1v |  |
| 2487 | x-fmt/385 | MPEG-1 Program Stream | mpeg, mpg | video/mpeg |
| 2488 | fmt/640 | MPEG-2 Elementary Stream | mpg, mpeg, m2v |  |
| 2489 | x-fmt/386 | MPEG-2 Program Stream | mpeg, mpg, mod | video/mpeg |
| 2490 | fmt/585 | MPEG-2 Transport Stream | m2t, ts, m2ts |  |
| 2491 | fmt/199 | MPEG-4 Media File | mp4, m4v, m4a, f4v, f4a | application/mp4, video/mp4 |
| 2492 | linguist/426 | MQL4 | .mq4, .mqh |  |
| 2493 | linguist/427 | MQL5 | .mq5, .mqh |  |
| 2494 | fmt/392 | MrSID Image Format (Multi-resolution Seamless Image Database) | sid |  |
| 2495 | fmt/469 | MS DOS Compression Format (KWAJ Variant) |  |  |
| 2496 | fmt/462 | MS-DOS Compression Format (SZDD Variant) |  |  |
| 2497 | x-fmt/409 | MS-DOS Executable | exe |  |
| 2498 | x-fmt/15 | MS-DOS Text File |  | text/plain |
| 2499 | x-fmt/130 | MS-DOS Text File with line breaks |  | text/plain |
| 2500 | linguist/218 | MTML | .mtml | text/html |
| 2501 | linguist/219 | MUF | .muf, .m | text/x-forth |
| 2502 | fmt/1471 | Multi Palette Picture File | mpp |  |
| 2503 | fmt/1468 | multiArtist File | mg1, mg2, mg4, mg8 |  |
| 2504 | x-fmt/347 | MultiMate Text File | dox, fnx, pat |  |
| 2505 | fmt/1800 | Multimedia Viewer Book | mvb |  |
| 2506 | x-fmt/348 | Multipage Zsoft Paintbrush Bitmap Graphics | dcx | image/x-dcx |
| 2507 | fmt/528 | Multiple-image Network Graphics | mng | video/x-mng |
| 2508 | fmt/719 | MultiTracker Module | mtm |  |
| 2509 | linguist/416 | mupad | .mu |  |
| 2510 | linguist/474864066 | Muse | .muse |  |
| 2511 | fmt/965 | Music Encoding Initiative | mei |  |
| 2512 | fmt/896 | MusicXML | xml, musicxml | application/vnd.recordare.musicxml+xml |
| 2513 | linguist/638334590 | Mustache | .mustache | text/x-smarty |
| 2514 | fmt/1386 | Muvee autoProducer Project File | mve |  |
| 2515 | fmt/1387 | Muvee autoProducer Project File | mvex |  |
| 2516 | fmt/1388 | Muvee Reveal Project File | rvl |  |
| 2517 | linguist/239 | Myghty | .myt |  |
| 2518 | fmt/1197 | MyISAM Indexes File | myi |  |
| 2519 | fmt/868 | MySQL Table Definition Format | frm |  |
| 2520 | linguist/775996197 | nanorc | .nanorc |  |
| 2521 | x-fmt/163 | NAP Metafile | nap |  |
| 2522 | linguist/178322513 | Nasal | .nas |  |
| 2523 | linguist/171666519 | NASL | .nasl, .inc |  |
| 2524 | fmt/365 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| 2525 | fmt/364 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| 2526 | fmt/366 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| 2527 | fmt/858 | Navisworks Document | nwd, nwc |  |
| 2528 | fmt/859 | Navisworks Document | nwd, nwc |  |
| 2529 | fmt/860 | Navisworks Document | nwd, nwc |  |
| 2530 | fmt/857 | Navisworks Document | nwd, nwc |  |
| 2531 | fmt/1280 | NCH Dictation Audio File | dct |  |
| 2532 | linguist/240 | NCL | .ncl |  |
| 2533 | linguist/521429430 | Nearley | .ne, .nearley |  |
| 2534 | fmt/1002 | Nearly Raw Raster Data | nrrd |  |
| 2535 | fmt/1003 | Nearly Raw Raster Data | nrrd |  |
| 2536 | fmt/1005 | Nearly Raw Raster Data | nrrd |  |
| 2537 | fmt/1004 | Nearly Raw Raster Data | nrrd |  |
| 2538 | fmt/1006 | Nearly Raw Raster Data | nrrd |  |
| 2539 | fmt/1963 | NEC Thermo Tracer Image File | tmp |  |
| 2540 | linguist/243 | Nemerle | .n |  |
| 2541 | fmt/1545 | NeoDesk Icon File | nic |  |
| 2542 | fmt/1540 | NeoDisk Icon File | nic |  |
| 2543 | linguist/481192983 | NEON | .neon |  |
| 2544 | fmt/1743 | Nero Burning ROM Image File | nrg |  |
| 2545 | fmt/1368 | Nero CoverDesigner File | ncd |  |
| 2546 | linguist/417 | nesC | .nc |  |
| 2547 | fmt/283 | netCDF-3 64-bit | nc, cdf | application/netcdf, application/x-netcdf |
| 2548 | fmt/282 | netCDF-3 Classic | nc, cdf | application/netcdf, application/x-netcdf |
| 2549 | linguist/244 | NetLinx | .axs, .axi |  |
| 2550 | linguist/245 | NetLinx+ERB | .axs.erb, .axi.erb |  |
| 2551 | linguist/246 | NetLogo | .nlogo | text/x-common-lisp |
| 2552 | fmt/1132 | Netscape Bookmark File Format | htm, html |  |
| 2553 | fmt/1551 | NetWare Loadable Module | nlm |  |
| 2554 | linguist/247 | NewLisp | .nl, .lisp, .lsp | text/x-common-lisp |
| 2555 | x-fmt/196 | NeXt Sound |  |  |
| 2556 | x-fmt/139 | NeXT/Sun sound | au | audio/basic |
| 2557 | linguist/506780613 | Nextflow | .nf |  |
| 2558 | linguist/248 | Nginx | .nginx, .nginxconf, .vhost | text/x-nginx-conf |
| 2559 | fmt/983 | NIB File Format | nib |  |
| 2560 | fmt/202 | Nikon Digital SLR Camera Raw Image File | nef, nrw |  |
| 2561 | linguist/249 | Nim | .nim, .nim.cfg, .nimble, .nimrod, .nims |  |
| 2562 | linguist/250 | Ninja | .ninja |  |
| 2563 | linguist/251 | Nit | .nit |  |
| 2564 | fmt/1166 | Niton Data Transfer | ndt |  |
| 2565 | linguist/252 | Nix | .nix |  |
| 2566 | linguist/241 | NL | .nl |  |
| 2567 | linguist/136456478 | NMODL | .mod |  |
| 2568 | fmt/1228 | NMRPipe | dat, pipe, ft2, ft3 |  |
| 2569 | fmt/1227 | NMRView | nv |  |
| 2570 | linguist/813068465 | Noir | .nr | text/x-rustsrc |
| 2571 | fmt/1896 | Nokia Picture Message | npm |  |
| 2572 | fmt/1902 | Norton Change Directory Persistent Cache File | ncd |  |
| 2573 | x-fmt/349 | Nota Bene Text File | nb |  |
| 2574 | fmt/974 | Notation Interchange File Format | nif | application/vnd.music-niff |
| 2575 | fmt/873 | Notation3 | n3 | text/n3 |
| 2576 | fmt/1486 | Novell Address Book | nab |  |
| 2577 | linguist/685022663 | NPM Config |  |  |
| 2578 | linguist/242 | NSIS | .nsi, .nsh | text/x-nsis |
| 2579 | fmt/1493 | NTI JewelCase Maker | jwc |  |
| 2580 | linguist/253 | Nu | .nu | text/x-scheme |
| 2581 | fmt/850 | NuFile Exchange Archival Library | shk, sdk, bxy |  |
| 2582 | fmt/644 | Nullsoft Scriptable Install System | nsi |  |
| 2583 | fmt/1176 | Nullsoft Streaming Video | nsv |  |
| 2584 | linguist/254 | NumPy | .numpy, .numpyw, .numsc | text/x-python |
| 2585 | linguist/461856962 | Nunjucks | .njk |  |
| 2586 | linguist/446573572 | Nushell | .nu | text/x-sh |
| 2587 | fmt/816 | NUT Open Container Format | nut |  |
| 2588 | linguist/731233819 | NWScript | .nss | text/x-csrc |
| 2589 | linguist/834374816 | OASv2-json | .json | application/json |
| 2590 | linguist/105187618 | OASv2-yaml | .yaml, .yml | text/x-yaml |
| 2591 | linguist/980062566 | OASv3-json | .json | application/json |
| 2592 | linguist/51239111 | OASv3-yaml | .yaml, .yml | text/x-yaml |
| 2593 | linguist/677210597 | Oberon | .ob2 |  |
| 2594 | linguist/256 | ObjDump | .objdump |  |
| 2595 | linguist/985227236 | Object Data Instance Notation | .odin |  |
| 2596 | linguist/257 | Objective-C | .m, .h | text/x-objectivec |
| 2597 | linguist/258 | Objective-C++ | .mm | text/x-objectivec |
| 2598 | linguist/259 | Objective-J | .j, .sj |  |
| 2599 | linguist/202735509 | ObjectScript | .cls |  |
| 2600 | fmt/1681 | OBO Flat File Format | obo |  |
| 2601 | fmt/207 | Obsidium Project File | opf |  |
| 2602 | linguist/255 | OCaml | .ml, .eliom, .eliomi, .ml4, .mli, .mll, .mly | text/x-ocaml |
| 2603 | linguist/889244082 | Odin | .odin |  |
| 2604 | fmt/1700 | OGC GeoPackage | gpkg | application/geopackage+sqlite3 |
| 2605 | fmt/947 | Ogg FLAC Compressed Multimedia File | ogg | audio/ogg |
| 2606 | fmt/944 | Ogg Multimedia Container | ogg, ogv, spx, opus | application/ogg |
| 2607 | fmt/946 | Ogg Opus Codec Compressed Multimedia File | ogg, opus | audio/ogg, audio/opus |
| 2608 | fmt/948 | Ogg Speex Codec Multimedia File | ogg, spx | audio/ogg, audio/speex |
| 2609 | fmt/945 | Ogg Theora Video | ogv, ogg | video/ogg |
| 2610 | fmt/203 | Ogg Vorbis Codec Compressed Multimedia File | ogg | audio/ogg |
| 2611 | fmt/1048 | OGR GFS File | gfs |  |
| 2612 | fmt/1188 | Ogre Mesh 1.x | mesh |  |
| 2613 | fmt/1189 | Ogre Mesh XML | xml |  |
| 2614 | fmt/722 | Oktalyzer Audio file | okt |  |
| 2615 | fmt/111 | OLE2 Compound Document Format |  |  |
| 2616 | fmt/668 | Olympus RAW | orf |  |
| 2617 | linguist/260 | Omgrofl | .omgrofl |  |
| 2618 | linguist/664100008 | omnetpp-msg | .msg |  |
| 2619 | linguist/924868392 | omnetpp-ned | .ned |  |
| 2620 | fmt/963 | OMNIC Spectral Data File | spa |  |
| 2621 | fmt/1373 | OmniPage Document | opd |  |
| 2622 | fmt/1372 | OmniPage Document | opd |  |
| 2623 | fmt/1371 | OmniPage Pro Document | opd |  |
| 2624 | x-fmt/350 | OmniPage Pro Document | met |  |
| 2625 | x-fmt/3 | Online Description Tool Format | odt |  |
| 2626 | linguist/418 | ooc | .ooc |  |
| 2627 | linguist/261 | Opa | .opa |  |
| 2628 | linguist/262 | Opal | .opal |  |
| 2629 | fmt/1889 | Open Access III Document | ext |  |
| 2630 | fmt/309 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| 2631 | fmt/313 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| 2632 | fmt/311 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| 2633 | fmt/310 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| 2634 | fmt/312 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| 2635 | fmt/832 | Open Inventor File Format | iv |  |
| 2636 | fmt/833 | Open Inventor File Format | iv |  |
| 2637 | fmt/1854 | Open Media Framework Interchange | omf |  |
| 2638 | fmt/1855 | Open Media Framework Interchange | omf |  |
| 2639 | linguist/840483232 | Open Policy Agent | .rego |  |
| 2640 | fmt/522 | Open Project File | pod |  |
| 2641 | fmt/657 | Open XML Paper Specification | xps, oxps | application/oxps |
| 2642 | linguist/848295328 | OpenAPI Specification v2 |  |  |
| 2643 | linguist/557959099 | OpenAPI Specification v3 |  |  |
| 2644 | linguist/263 | OpenCL | .cl, .opencl | text/x-csrc |
| 2645 | fmt/1752 | OpenDocument Database Format | odb | application/vnd.oasis.opendocument.base |
| 2646 | fmt/444 | OpenDocument Database Format | odb |  |
| 2647 | fmt/140 | OpenDocument Database Format | odb |  |
| 2648 | fmt/424 | OpenDocument Database Format | odb |  |
| 2649 | fmt/135 | OpenDocument Format |  |  |
| 2650 | fmt/297 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| 2651 | fmt/139 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| 2652 | fmt/296 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| 2653 | fmt/1753 | OpenDocument Graphics | odg | application/vnd.oasis.opendocument.graphics |
| 2654 | fmt/292 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| 2655 | fmt/138 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| 2656 | fmt/1754 | OpenDocument Presentation | odp | application/vnd.oasis.opendocument.presentation |
| 2657 | fmt/293 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| 2658 | fmt/295 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| 2659 | fmt/294 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| 2660 | fmt/1755 | OpenDocument Spreadsheet | ods | application/vnd.oasis.opendocument.spreadsheet |
| 2661 | fmt/137 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| 2662 | fmt/136 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| 2663 | fmt/291 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| 2664 | fmt/290 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| 2665 | fmt/1756 | OpenDocument Text | odt | application/vnd.oasis.opendocument.text |
| 2666 | linguist/264 | OpenEdge ABL | .p, .cls, .w |  |
| 2667 | fmt/1001 | OpenEXR | exr | image/x-exr |
| 2668 | fmt/129 | OpenOffice Calc | sxc | application/vnd.sun.xml.calc |
| 2669 | fmt/127 | OpenOffice Draw | sxd | application/vnd.sun.xml.draw |
| 2670 | fmt/130 | OpenOffice Impress | sxi | application/vnd.sun.xml.impress |
| 2671 | fmt/128 | OpenOffice Writer | sxw | application/vnd.sun.xml.writer |
| 2672 | linguist/153739399 | OpenQASM | .qasm |  |
| 2673 | fmt/998 | OpenRaster Image Format | ora | image/openraster |
| 2674 | linguist/265 | OpenRC runscript |  | text/x-sh |
| 2675 | linguist/266 | OpenSCAD | .scad |  |
| 2676 | linguist/598917541 | OpenStep Property List | .plist, .glyphs |  |
| 2677 | linguist/374317347 | OpenType Feature File | .fea |  |
| 2678 | fmt/520 | OpenType Font File | otf | font/otf |
| 2679 | fmt/1947 | OpenWayback CDXJ File Format | cdx, cdxj |  |
| 2680 | fmt/1883 | OPML File | opml |  |
| 2681 | fmt/1882 | OPML File | opml |  |
| 2682 | fmt/695 | Optimised Dalvik Executable Format | odex |  |
| 2683 | linguist/723589315 | Option List |  | text/x-sh |
| 2684 | fmt/1465 | OrCAD Layout File | max |  |
| 2685 | fmt/1572 | OrCAD Project File | opj |  |
| 2686 | linguist/267 | Org | .org |  |
| 2687 | fmt/1457 | OrgPlus File | opx, opxt, ops |  |
| 2688 | fmt/1124 | Origin Project Format | opju, oggu, ogmu, ogwu |  |
| 2689 | fmt/1123 | Origin Project Format | opj, ogg, ogm, ogw |  |
| 2690 | x-fmt/25 | OS/2 Bitmap |  |  |
| 2691 | x-fmt/270 | OS/2 Bitmap | bmp | image/bmp |
| 2692 | x-fmt/143 | OS/2 Change Control File | cin |  |
| 2693 | x-fmt/67 | OS/2 Presentation Manager Metafile (MET) | met |  |
| 2694 | fmt/839 | Outlook Express Folder Database | dbx |  |
| 2695 | fmt/838 | Outlook Express Message Database | dbx |  |
| 2696 | linguist/268 | Ox | .ox, .oxh, .oxo |  |
| 2697 | linguist/269 | Oxygene | .oxygene |  |
| 2698 | linguist/270 | Oz | .oz | text/x-oz |
| 2699 | fmt/823 | P00 C64 Image Format | p00, p01, p02, p03, p04 |  |
| 2700 | linguist/348895984 | P4 | .p4 |  |
| 2701 | fmt/1721 | Pablo Paint Raster Image | ppp, pa3 |  |
| 2702 | fmt/1608 | Packed-Ice True Colour Picture [Spooky Sprites] (Atari Falcon) | trp, tru |  |
| 2703 | fmt/1606 | Packed-Ice True Colour Sprites [Spooky Sprites] (Atari Falcon) | trs |  |
| 2704 | linguist/756774415 | Pact | .pact |  |
| 2705 | x-fmt/351 | PageMaker Document | pm3 |  |
| 2706 | fmt/876 | Pagemaker Document (Generic) | p65, pmd, pmt | application/vnd.pagemaker |
| 2707 | fmt/1719 | PageMaker Mac Document | pm6, pt6 | application/vnd.pagemaker |
| 2708 | fmt/1718 | PageMaker Mac Document | p65, t65, pmd, pmt | application/vnd.pagemaker |
| 2709 | fmt/1687 | PageMaker Mac Document |  | application/vnd.pagemaker |
| 2710 | fmt/1686 | PageMaker Mac Document |  | application/vnd.pagemaker |
| 2711 | x-fmt/173 | PageMaker PC Document | pm5, pt5 | application/vnd.pagemaker |
| 2712 | x-fmt/174 | PageMaker PC Document | pm6, pt6 | application/vnd.pagemaker |
| 2713 | x-fmt/181 | PageMaker PC Document | p65, t65, pmd, pmt | application/vnd.pagemaker |
| 2714 | x-fmt/352 | PageMaker PC Document | pm4, pt4 | application/vnd.pagemaker |
| 2715 | x-fmt/198 | Pagemaker TableEditor Graphics | tbl |  |
| 2716 | fmt/1597 | PageMaker Template File | pt5 |  |
| 2717 | x-fmt/200 | PageMaker Time Stamp File | tym |  |
| 2718 | fmt/348 | Paint Shop Pro Image | pspimage |  |
| 2719 | x-fmt/233 | Paint Shop Pro Image | psp |  |
| 2720 | x-fmt/376 | Paint Shop Pro Image | pspimage |  |
| 2721 | x-fmt/234 | Paint Shop Pro Image | psp |  |
| 2722 | fmt/349 | Paint Shop Pro Image | pspimage |  |
| 2723 | x-fmt/377 | Paint Shop Pro Image | psp |  |
| 2724 | x-fmt/297 | Paint Shop Pro Image | psp, pspimage |  |
| 2725 | x-fmt/298 | Paint Shop Pro Image | psp, pspimage |  |
| 2726 | x-fmt/187 | Painter RIFF Image File | rif |  |
| 2727 | fmt/1733 | PaintShop Plus Compressed Format | psc, da4 |  |
| 2728 | fmt/217 | PaintShop Pro Browser Cache File | jbf |  |
| 2729 | fmt/1654 | Palm Database ImageViewer Format | pdb |  |
| 2730 | linguist/276 | Pan | .pan |  |
| 2731 | fmt/662 | Panasonic Raw | rw2 |  |
| 2732 | fmt/1339 | PaperPort MAX | max |  |
| 2733 | fmt/1224 | PaperPort MAX | max |  |
| 2734 | fmt/1223 | PaperPort MAX | max |  |
| 2735 | fmt/1225 | PaperPort MAX | max |  |
| 2736 | linguist/277 | Papyrus | .psc |  |
| 2737 | fmt/1965 | Papyrus Document | pap, pav, pbf |  |
| 2738 | x-fmt/307 | Paradox Database Memo Field (Binary Large Object) | dbq, mb |  |
| 2739 | x-fmt/147 | Paradox Database Table | db |  |
| 2740 | fmt/351 | Paradox Database Table | db |  |
| 2741 | fmt/350 | Paradox Database Table | db |  |
| 2742 | fmt/352 | Paradox Database Table | db |  |
| 2743 | linguist/278 | Parrot | .parrot |  |
| 2744 | linguist/279 | Parrot Assembly | .pasm |  |
| 2745 | linguist/280 | Parrot Internal Representation | .pir |  |
| 2746 | linguist/281 | Pascal | .pas, .dfm, .dpr, .inc, .lpr, .pascal, .pp | text/x-pascal |
| 2747 | fmt/1619 | Pascal Source Code | pas |  |
| 2748 | fmt/1904 | Pasti Floppy Disk Image | stx |  |
| 2749 | linguist/271 | Pawn | .pwn, .inc, .sma |  |
| 2750 | x-fmt/170 | PC Paint Bitmap | pic |  |
| 2751 | fmt/780 | pcap Next Generation Packet Capture | pcapng | application/vnd.tcpdump.pcap |
| 2752 | fmt/779 | pcap Packet Capture | pcap, cap, dmp | application/vnd.tcpdump.pcap |
| 2753 | fmt/1936 | PCRaster | csf, map |  |
| 2754 | fmt/90 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| 2755 | fmt/86 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| 2756 | fmt/89 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| 2757 | fmt/87 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| 2758 | fmt/88 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| 2759 | linguist/736235603 | PDDL | .pddl |  |
| 2760 | fmt/1129 | PDF 2.0 - Portable Document Format | pdf | application/pdf |
| 2761 | fmt/1451 | PDF Portfolio | pdf |  |
| 2762 | fmt/1095 | PEA Archive Format | pea |  |
| 2763 | fmt/330 | Peak Graphical Waveform File | pk |  |
| 2764 | fmt/1952 | PechaMaker Format | pxp |  |
| 2765 | linguist/81442128 | PEG.js | .pegjs, .peggy | text/javascript |
| 2766 | fmt/1781 | Pentax PEF Image File | pef | image/dng |
| 2767 | linguist/840372442 | Pep8 | .pep |  |
| 2768 | fmt/1898 | Perfect ZX Tape (PZX) Image Format | pzx |  |
| 2769 | linguist/282 | Perl | .pl, .al, .cgi, .fcgi, .perl, .ph, .plx, .pm, .psgi, .t | text/x-perl |
| 2770 | fmt/870 | Perl Script | pl |  |
| 2771 | fmt/855 | Personal Ancestral File (PAF) | paf |  |
| 2772 | fmt/856 | Personal Ancestral File (PAF) | paf |  |
| 2773 | fmt/854 | Personal Ancestral File (PAF) | paf |  |
| 2774 | fmt/1710 | Persuasion Auto-Template Interchange File | atf |  |
| 2775 | fmt/1705 | Persuasion Mac Document | pn4 |  |
| 2776 | fmt/1702 | Persuasion Mac Document | pr2 |  |
| 2777 | fmt/1701 | Persuasion Mac Document | pr1 |  |
| 2778 | fmt/1704 | Persuasion Mac Document | pr3 |  |
| 2779 | fmt/1703 | Persuasion Mac Document | pr2 |  |
| 2780 | fmt/1708 | Persuasion Player File | ppf |  |
| 2781 | fmt/1709 | Persuasion Presentation Interchange File | prf |  |
| 2782 | fmt/1706 | Persuasion Windows Document | pr2, at2 |  |
| 2783 | fmt/1707 | Persuasion Windows Document | pr3, at3, pn4, at4 |  |
| 2784 | fmt/1284 | PFS:First Choice Database | fol |  |
| 2785 | fmt/1283 | PFS:First Choice Document | doc |  |
| 2786 | fmt/1282 | PFS:First Choice Document | doc |  |
| 2787 | fmt/1285 | PFS:First Choice Graph | gra |  |
| 2788 | fmt/1414 | PFS:Write Document | pfs |  |
| 2789 | fmt/1489 | Phantom CINE Compressed Video File | cci |  |
| 2790 | fmt/1488 | Phantom CINE Video File | cine, cin |  |
| 2791 | fmt/1061 | Phase One IIQ Raw Image | iiq |  |
| 2792 | fmt/1060 | Phase One Raw Image | cap, capture |  |
| 2793 | fmt/667 | Photoshop Curve File | acv, atf |  |
| 2794 | linguist/272 | PHP | .php, .aw, .ctp, .fcgi, .inc, .php3, .php4, .php5, .phps, .phpt | application/x-httpd-php |
| 2795 | x-fmt/169 | PHP Script Page | php | text/html |
| 2796 | linguist/425 | Pic | .pic, .chem | text/troff |
| 2797 | linguist/284 | Pickle | .pkl |  |
| 2798 | linguist/285 | PicoLisp | .l |  |
| 2799 | x-fmt/166 | PICS Animation | pcs |  |
| 2800 | x-fmt/85 | Picture Publisher Bitmap | pp5 |  |
| 2801 | x-fmt/176 | Picture Publisher Bitmap | pp4 |  |
| 2802 | fmt/1360 | Picture Publisher Bitmap | ppf |  |
| 2803 | linguist/286 | PigLatin | .pig |  |
| 2804 | linguist/287 | Pike | .pike, .pmod |  |
| 2805 | linguist/684385621 | Pip Requirements |  |  |
| 2806 | fmt/1745 | PixArt Bitmap | pix |  |
| 2807 | fmt/670 | PKCS #7 Cryptographic Message File | p7m, p7b, p7s | application/pkcs7-mime, application/pkcs7-signature |
| 2808 | linguist/288822799 | Pkl | .pkl |  |
| 2809 | x-fmt/111 | Plain Text File | txt | text/plain |
| 2810 | linguist/833504686 | PlantUML | .puml, .iuml, .plantuml |  |
| 2811 | fmt/314 | Play SID Audio | sid, psid | audio/prs.sid |
| 2812 | fmt/315 | Play SID Audio | sid, psid | audio/prs.sid |
| 2813 | linguist/274 | PLpgSQL | .pgsql, .sql | text/x-sql |
| 2814 | linguist/273 | PLSQL | .pls, .bdy, .ddl, .fnc, .pck, .pkb, .pks, .plb, .plsql, .prc, .spc, .sql, .tpb, .tps, .trg, .vw | text/x-plsql |
| 2815 | x-fmt/94 | Pocket Word Document | psw, pwd |  |
| 2816 | x-fmt/96 | Pocket Word Template | pwt |  |
| 2817 | fmt/396 | PocketMobi (Palm Resource) File | mobi, prc |  |
| 2818 | linguist/288 | Pod | .pod | text/x-perl |
| 2819 | linguist/155357471 | Pod 6 | .pod, .pod6 |  |
| 2820 | linguist/289 | PogoScript | .pogo |  |
| 2821 | linguist/839112914 | Polar | .polar |  |
| 2822 | fmt/831 | Polygon File Format | ply |  |
| 2823 | fmt/519 | Polynomial Texture Map | ptm |  |
| 2824 | linguist/290 | Pony | .pony |  |
| 2825 | fmt/405 | Portable Any Map | pam |  |
| 2826 | x-fmt/164 | Portable Bitmap Image - ASCII | pbm | image/x-portable-bitmap |
| 2827 | fmt/409 | Portable Bitmap Image - Binary | pbmb, pnm |  |
| 2828 | fmt/1720 | Portable Compiled Format | pcf |  |
| 2829 | fmt/1066 | Portable Database | pdb |  |
| 2830 | fmt/1065 | Portable Database | pdb |  |
| 2831 | fmt/1064 | Portable Database | pdb |  |
| 2832 | fmt/322 | Portable Form File | pff |  |
| 2833 | fmt/407 | Portable Grey Map - ASCII | pgma, pgm |  |
| 2834 | fmt/406 | Portable Grey Map - Binary | pgmb, pgm |  |
| 2835 | fmt/11 | Portable Network Graphics | png | image/png |
| 2836 | fmt/13 | Portable Network Graphics | png | image/png |
| 2837 | fmt/12 | Portable Network Graphics | png | image/png |
| 2838 | x-fmt/178 | Portable Pixel Map - ASCII | ppm | image/x-portable-pixmap |
| 2839 | fmt/408 | Portable Pixel Map - Binary | ppm, ppmb |  |
| 2840 | fmt/959 | Portable Sound Format | psf, psf1, psflib, minipsf, minipsf1, gsf, gsflib, minigsf |  |
| 2841 | fmt/1734 | Portfolio Graphics Compressed File | pgc |  |
| 2842 | linguist/832391833 | Portugol | .por |  |
| 2843 | linguist/262764437 | PostCSS | .pcss, .postcss |  |
| 2844 | x-fmt/406 | PostScript | ps | application/postscript |
| 2845 | x-fmt/408 | PostScript | ps | application/postscript |
| 2846 | linguist/291 | PostScript | .ps, .eps, .epsi, .pfa |  |
| 2847 | fmt/501 | PostScript | ps | application/postscript |
| 2848 | x-fmt/407 | PostScript | ps | application/postscript |
| 2849 | x-fmt/91 | Postscript | ps | application/postscript |
| 2850 | x-fmt/93 | Postscript Support File | psf |  |
| 2851 | linguist/275 | POV-Ray SDL | .pov, .inc |  |
| 2852 | linguist/292 | PowerBuilder | .pbt, .sra, .sru, .srw |  |
| 2853 | fmt/1201 | PowerCADD |  |  |
| 2854 | fmt/1200 | PowerDraw |  |  |
| 2855 | fmt/1731 | PowerGraphics Image File | pgr |  |
| 2856 | fmt/512 | PowerProject | pp |  |
| 2857 | fmt/515 | PowerProject | pp |  |
| 2858 | fmt/516 | PowerProject | pp |  |
| 2859 | fmt/511 | PowerProject | pp |  |
| 2860 | fmt/517 | PowerProject | pp |  |
| 2861 | fmt/514 | PowerProject | pp |  |
| 2862 | fmt/513 | PowerProject | pp |  |
| 2863 | fmt/510 | PowerProject Teamplan | pdb |  |
| 2864 | linguist/293 | PowerShell | .ps1, .psd1, .psm1 | application/x-powershell |
| 2865 | fmt/782 | PowerVR Object Data | pod |  |
| 2866 | fmt/1829 | PPTX Strict OOXML Presentation | pptx | application/vnd.openxmlformats-officedocument.presentationml.presentation |
| 2867 | linguist/106029007 | Praat | .praat |  |
| 2868 | fmt/1164 | Praat Picture File | prapic |  |
| 2869 | fmt/1165 | Praat Script File | praat |  |
| 2870 | fmt/1801 | Praat TextGrid | textgrid | text/praat-textgrid |
| 2871 | fmt/1070 | Preferred Executable Format |  |  |
| 2872 | fmt/1455 | Primavera P6 Project Management XER File | xer |  |
| 2873 | fmt/185 | Prime OCR | pro |  |
| 2874 | fmt/187 | PrimeOCR | pro |  |
| 2875 | fmt/188 | PrimeOCR | pro |  |
| 2876 | fmt/184 | PrimeOCR | pro |  |
| 2877 | fmt/186 | PrimeOCR | pro |  |
| 2878 | fmt/183 | PrimeOCR | pro |  |
| 2879 | fmt/1302 | PrintMaster Gold Project | ban, cal, car, let, sig |  |
| 2880 | fmt/1732 | Prism Paint Bitmap | pnt, tpi |  |
| 2881 | linguist/499933428 | Prisma | .prisma |  |
| 2882 | fmt/1727 | Pro Tools Session File | ptx |  |
| 2883 | fmt/1951 | Pro Tools Session File | ptf, pts |  |
| 2884 | linguist/294 | Processing | .pde |  |
| 2885 | fmt/701 | Processing Development Environment | pde |  |
| 2886 | linguist/305313959 | Procfile |  |  |
| 2887 | x-fmt/353 | Professional Write Text File | pw |  |
| 2888 | fmt/1957 | Program Embroidery Stitch (PES) File | pes |  |
| 2889 | fmt/1128 | Progressive Graphics File | pgf |  |
| 2890 | linguist/716513858 | Proguard | .pro |  |
| 2891 | linguist/295 | Prolog | .pl, .plt, .pro, .prolog, .yap |  |
| 2892 | linguist/441858312 | Promela | .pml |  |
| 2893 | linguist/296 | Propeller Spin | .spin |  |
| 2894 | fmt/2009 | Protein Data Bank File | pdb |  |
| 2895 | linguist/297 | Protocol Buffer | .proto | text/x-protobuf |
| 2896 | linguist/436568854 | Protocol Buffer Text Format | .textproto, .pbt, .pbtxt |  |
| 2897 | fmt/1744 | Psion Series 3 Bitmap | pic |  |
| 2898 | fmt/1897 | Ptex File Format | ptx |  |
| 2899 | fmt/1343 | PTGui Project File | pts |  |
| 2900 | fmt/1344 | PTGui Project File | pts |  |
| 2901 | linguist/298 | Public Key | .asc, .pub | application/pgp |
| 2902 | linguist/179 | Pug | .jade, .pug | text/x-pug |
| 2903 | fmt/360 | pulse EKKO data file | dt1 |  |
| 2904 | fmt/361 | pulse EKKO header file | hd |  |
| 2905 | linguist/299 | Puppet | .pp | text/x-puppet |
| 2906 | linguist/300 | Pure Data | .pd |  |
| 2907 | linguist/301 | PureBasic | .pb, .pbi |  |
| 2908 | linguist/302 | PureScript | .purs | text/x-haskell |
| 2909 | linguist/252961827 | Pyret | .arr |  |
| 2910 | linguist/303 | Python | .py, .cgi, .fcgi, .gyp, .gypi, .lmi, .py3, .pyde, .pyi, .pyp, .pyt, .pyw, .rpy, .spec, .tac, .wsgi, .xpy | text/x-python |
| 2911 | fmt/1115 | Python Compiled File | pyc |  |
| 2912 | fmt/1117 | Python Compiled File | pyc |  |
| 2913 | fmt/1112 | Python Compiled File | pyc |  |
| 2914 | fmt/940 | Python Compiled File | pyc |  |
| 2915 | fmt/1116 | Python Compiled File | pyc |  |
| 2916 | fmt/1107 | Python Compiled File | pyc |  |
| 2917 | fmt/1110 | Python Compiled File | pyc |  |
| 2918 | fmt/1114 | Python Compiled File | pyc |  |
| 2919 | fmt/1108 | Python Compiled File | pyc |  |
| 2920 | fmt/1113 | Python Compiled File | pyc |  |
| 2921 | fmt/1109 | Python Compiled File | pyc |  |
| 2922 | fmt/1118 | Python Compiled File | pyc |  |
| 2923 | fmt/939 | Python Compiled File | pyc |  |
| 2924 | fmt/1106 | Python Compiled File | pyc |  |
| 2925 | fmt/1111 | Python Compiled File | pyc |  |
| 2926 | linguist/428 | Python console |  |  |
| 2927 | fmt/938 | Python Source Code File | py |  |
| 2928 | linguist/304 | Python traceback | .pytb |  |
| 2929 | linguist/970539067 | q | .q |  |
| 2930 | linguist/697448245 | Q# | .qs |  |
| 2931 | fmt/1045 | Q&A Word Processor Document |  |  |
| 2932 | fmt/962 | QCP Audio File Format | qcp | audio/qcelp |
| 2933 | linguist/306 | QMake | .pro, .pri |  |
| 2934 | linguist/305 | QML | .qml, .qbs |  |
| 2935 | fmt/830 | Qsplat Model | qs |  |
| 2936 | linguist/558193693 | Qt Script | .qs | text/javascript |
| 2937 | fmt/888 | QuadriSpace Format | qsd, qsl, qsm, qst |  |
| 2938 | linguist/375265331 | Quake |  |  |
| 2939 | x-fmt/182 | QuarkXPress Data File | qxd, qxt, qxp, qcd, qxl, qxb, qwd, qwt, qpt | application/vnd.Quark.QuarkXPress |
| 2940 | fmt/1317 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| 2941 | fmt/1444 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| 2942 | fmt/1320 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| 2943 | fmt/1442 | QuarkXPress Document |  | application/vnd.Quark.QuarkXPress |
| 2944 | fmt/1443 | QuarkXPress Document |  | application/vnd.Quark.QuarkXPress |
| 2945 | fmt/1319 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| 2946 | fmt/1318 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| 2947 | fmt/1327 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2948 | fmt/1321 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2949 | fmt/1445 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2950 | fmt/1325 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2951 | fmt/1326 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2952 | fmt/1494 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2953 | fmt/1328 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2954 | fmt/1322 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2955 | fmt/685 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| 2956 | fmt/2008 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| 2957 | fmt/652 | QuarkXPress Project | qpt, qwd, qxp | application/vnd.Quark.QuarkXPress |
| 2958 | fmt/1324 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2959 | fmt/1323 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2960 | fmt/1495 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| 2961 | fmt/1446 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| 2962 | fmt/2007 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| 2963 | fmt/651 | QuarkXPress Project | qpt, qwd, qxp | application/vnd.Quark.QuarkXPress |
| 2964 | fmt/2006 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| 2965 | fmt/650 | QuarkXPress Report File | qxp report, xtg, qxp%20report | application/vnd.Quark.QuarkXPress |
| 2966 | fmt/508 | Quarter Inch Cartridge Host Interchange Format | qic |  |
| 2967 | fmt/836 | Quattro Pro Spreadsheet | wb3 |  |
| 2968 | fmt/837 | Quattro Pro Spreadsheet | qpw |  |
| 2969 | x-fmt/121 | Quattro Pro Spreadsheet for DOS | wq1, wkq |  |
| 2970 | x-fmt/122 | Quattro Pro Spreadsheet for DOS | wq2, wkq |  |
| 2971 | fmt/834 | Quattro Pro Spreadsheet for Windows | wb1 |  |
| 2972 | fmt/835 | Quattro Pro Spreadsheet for Windows | wb2 |  |
| 2973 | linguist/593107205 | QuickBASIC | .bas | text/x-vb |
| 2974 | fmt/1354 | QuickBooks Backup File | qbb |  |
| 2975 | fmt/1049 | QuickDraw 3D Metafile (ASCII) | 3dmf |  |
| 2976 | fmt/1203 | QuickDraw 3D Metafile (Binary) | 3dmf |  |
| 2977 | fmt/1050 | QuickDraw 3D Metafile (Binary) | 3dmf |  |
| 2978 | fmt/1861 | Quicken 3 Database File | qst |  |
| 2979 | x-fmt/213 | Quicken Data File | abd, qdf, qel |  |
| 2980 | fmt/308 | Quicken Data Format | qdf |  |
| 2981 | fmt/307 | Quicken Interchange Format | qif | application/qif |
| 2982 | x-fmt/384 | Quicktime | mov, qtm | video/quicktime |
| 2983 | linguist/307 | R | .r, .rd, .rsx | text/x-rsrc |
| 2984 | fmt/1599 | R Program File | r |  |
| 2985 | linguist/316 | Racket | .rkt, .rktd, .rktl, .scrbl |  |
| 2986 | fmt/591 | Radiance RGBE Image Format | hdr, pic, rgbe, xyze | image/vnd.radiance |
| 2987 | linguist/317 | Ragel | .rl |  |
| 2988 | fmt/1894 | RagTime Document File |  |  |
| 2989 | fmt/1895 | RagTime Document File | rtd, rtt |  |
| 2990 | linguist/283 | Raku | .6pl, .6pm, .nqp, .p6, .p6l, .p6m, .pl, .pl6, .pm, .pm6, .raku, .rakumod, .t | text/x-perl |
| 2991 | linguist/308 | RAML | .raml | text/x-yaml |
| 2992 | fmt/411 | RAR Archive | rar | application/vnd.rar |
| 2993 | fmt/613 | RAR Archive | rar | application/vnd.rar |
| 2994 | x-fmt/264 | RAR Archive | rar | application/vnd.rar |
| 2995 | linguist/173616037 | Rascal | .rsc |  |
| 2996 | fmt/1539 | Raster Matrix Format | rsw |  |
| 2997 | x-fmt/185 | Raw Bitmap | raw |  |
| 2998 | fmt/1252 | Raw Flux Image | rfi |  |
| 2999 | fmt/41 | Raw JPEG Stream | jpe, jpg, jpeg, jif, jfif, jfi | image/jpeg |
| 3000 | fmt/1810 | Raw PIMA SWIR Reflectance Spectral File | fos |  |
| 3001 | linguist/318 | Raw token data | .raw |  |
| 3002 | linguist/899227493 | RBS | .rbs | text/x-ruby |
| 3003 | fmt/1198 | RData | rdata |  |
| 3004 | fmt/1199 | RData | rdata |  |
| 3005 | fmt/875 | RDF/XML | rdf | application/rdf+xml |
| 3006 | linguist/309 | RDoc | .rdoc |  |
| 3007 | linguist/538732839 | Readline Config |  |  |
| 3008 | fmt/316 | Real SID Audio | sid | audio/prs.sid |
| 3009 | x-fmt/277 | Real Video | rv | video/vnd.rn-realvideo |
| 3010 | fmt/404 | RealAudio | ra |  |
| 3011 | x-fmt/278 | RealAudio | ra | audio/vnd.rn-realaudio |
| 3012 | x-fmt/183 | RealAudio Metafile | ram | audio/vnd.rn-realaudio, audio/x-pn-realaudio |
| 3013 | linguist/310 | REALbasic | .rbbas, .rbfrm, .rbmnu, .rbres, .rbtbar, .rbuistate |  |
| 3014 | fmt/728 | RealLegal E-Transcript | ptx |  |
| 3015 | x-fmt/190 | RealMedia | rm, rmvb | application/vnd.rn-realmedia |
| 3016 | fmt/204 | RealVideo Clip | rv |  |
| 3017 | linguist/869538413 | Reason | .re, .rei | text/x-rustsrc |
| 3018 | linguist/319002153 | ReasonLIGO | .religo | text/x-rustsrc |
| 3019 | linguist/319 | Rebol | .reb, .r, .r2, .r3, .rebol |  |
| 3020 | linguist/865765202 | Record Jar |  | text/x-properties |
| 3021 | linguist/320 | Red | .red, .reds |  |
| 3022 | fmt/1664 | RED Thumbnail File | rtn |  |
| 3023 | linguist/321 | Redcode | .cw |  |
| 3024 | fmt/1039 | Redcode Metadata (RMD) File | rmd |  |
| 3025 | fmt/1038 | Redcode RAW (R3D) Media File | r3d |  |
| 3026 | fmt/588 | Redcode RAW (R3D) Media File | r3d |  |
| 3027 | linguist/1020148948 | Redirect Rules |  |  |
| 3028 | fmt/1215 | Reduced Resolution Dataset | img, ovr, rrd, aux, aoi, cff, fft, gcc, sig, sml |  |
| 3029 | linguist/363378884 | Regular Expression | .regexp, .regex |  |
| 3030 | linguist/322 | Ren'Py | .rpy |  |
| 3031 | linguist/323 | RenderScript | .rs, .rsh |  |
| 3032 | linguist/501875647 | ReScript | .res | text/x-rustsrc |
| 3033 | fmt/1886 | Resource Interchange File Format (RIFF) |  |  |
| 3034 | fmt/1565 | reStructuredText | rst |  |
| 3035 | linguist/419 | reStructuredText | .rst, .rest, .rest.txt, .rst.txt | text/x-rst |
| 3036 | x-fmt/11 | Revisable-Form-Text Document Content Architecture |  |  |
| 3037 | x-fmt/446 | Revit External Group | rvg | application/octet-stream |
| 3038 | x-fmt/443 | Revit Family File | rfa | application/octet-stream |
| 3039 | x-fmt/444 | Revit Family Template | rft | application/octet-stream |
| 3040 | x-fmt/447 | Revit Project | rvt | application/octet-stream |
| 3041 | x-fmt/445 | Revit Template | rte | application/octet-stream |
| 3042 | x-fmt/448 | Revit Workspace | rws | application/octet-stream |
| 3043 | fmt/1919 | Revolution Stack | rev, livecode |  |
| 3044 | linguist/311 | REXX | .rexx, .pprx, .rex |  |
| 3045 | linguist/498022874 | Rez | .r |  |
| 3046 | fmt/712 | RF64 | wav, rf64 |  |
| 3047 | fmt/713 | RF64 Multichannel Broadcast Wave format | wav, rf64 |  |
| 3048 | fmt/1290 | RFFlow Chart | flo |  |
| 3049 | fmt/1289 | RFFlow Chart | flo |  |
| 3050 | fmt/1291 | RFFlow Chart | flo |  |
| 3051 | fmt/969 | Rich Text Format | rtf | application/rtf |
| 3052 | fmt/46 | Rich Text Format |  |  |
| 3053 | fmt/53 | Rich Text Format | rtf | application/rtf, text/rtf |
| 3054 | fmt/50 | Rich Text Format | rtf | application/rtf, text/rtf |
| 3055 | fmt/51 | Rich Text Format |  |  |
| 3056 | fmt/49 | Rich Text Format |  |  |
| 3057 | linguist/51601661 | Rich Text Format | .rtf |  |
| 3058 | fmt/45 | Rich Text Format | rtf | application/rtf, text/rtf |
| 3059 | fmt/48 | Rich Text Format |  |  |
| 3060 | fmt/47 | Rich Text Format |  |  |
| 3061 | fmt/52 | Rich Text Format | rtf | application/rtf, text/rtf |
| 3062 | fmt/355 | Rich Text Format | rtf | application/rtf, text/rtf |
| 3063 | fmt/624 | RIFF Palette Format | pal |  |
| 3064 | fmt/956 | RIFF-based MIDI | rmi |  |
| 3065 | linguist/431 | Ring | .ring |  |
| 3066 | linguist/878396783 | Riot | .riot |  |
| 3067 | fmt/1899 | RIS Citation | ris | application/x-research-info-systems |
| 3068 | linguist/313 | RMarkdown | .qmd, .rmd | text/x-gfm |
| 3069 | linguist/324 | RobotFramework | .robot, .resource |  |
| 3070 | linguist/674736065 | robots.txt |  |  |
| 3071 | linguist/440182480 | Roc | .roc |  |
| 3072 | fmt/485 | Rocket Book eBook format | rb |  |
| 3073 | fmt/1746 | Rocky Interlace Picture | rip |  |
| 3074 | linguist/141 | Roff | .roff, .1, .1in, .1m, .1x, .2, .3, .3in, .3m, .3p, .3pm, .3qt, .3x, .4, .5, .6, .7, .8, .9, .l, .man, .mdoc, .me, .ms, .n, .nr, .rno, .tmac | text/troff |
| 3075 | linguist/612669833 | Roff Manpage | .1, .1in, .1m, .1x, .2, .3, .3in, .3m, .3p, .3pm, .3qt, .3x, .4, .5, .6, .7, .8, .9, .man, .mdoc | text/troff |
| 3076 | linguist/587855233 | RON | .ron |  |
| 3077 | fmt/1338 | RootsMagic Database | rmgc |  |
| 3078 | linguist/325 | Rouge | .rg | text/x-clojure |
| 3079 | linguist/592853203 | RouterOS Script | .rsc |  |
| 3080 | fmt/1670 | Roxio Audio Project File | rox |  |
| 3081 | fmt/1669 | Roxio Data Project File | rox |  |
| 3082 | fmt/1667 | Roxio Easy Media Creator - Classic Creator File | rcl |  |
| 3083 | fmt/1666 | Roxio Easy Media Creator Layout | rcl |  |
| 3084 | fmt/1668 | Roxio Easy Media Creator Layout | roxio |  |
| 3085 | fmt/1645 | Roxio Label Creator Project File | jwl |  |
| 3086 | fmt/1646 | Roxio Label Creator Project File | jwl |  |
| 3087 | fmt/1644 | Roxio Label Creator Project File | jwl |  |
| 3088 | linguist/1031374237 | RPC | .x |  |
| 3089 | linguist/609977990 | RPGLE | .rpgle, .sqlrpgle |  |
| 3090 | fmt/794 | RPM Package Manager file | rpm, src.rpm |  |
| 3091 | fmt/793 | RPM Package Manager file | rpm, src.rpm |  |
| 3092 | fmt/795 | RPM Package Manager file | rpm, src.rpm |  |
| 3093 | linguist/314 | RPM Spec | .spec | text/x-rpm-spec |
| 3094 | linguist/326 | Ruby | .rb, .builder, .eye, .fcgi, .gemspec, .god, .jbuilder, .mspec, .pluginspec, .podspec, .prawn, .rabl, .rake, .rbi, .rbuild, .rbw, .rbx, .ru, .ruby, .spec, .thor, .watchr | text/x-ruby |
| 3095 | linguist/315 | RUNOFF | .rnh, .rno |  |
| 3096 | linguist/327 | Rust | .rs, .rs.in | text/x-rustsrc |
| 3097 | fmt/1935 | S-57 Electronic Navigational Chart | 000, 001, 002, 003, 004, 006 |  |
| 3098 | fmt/887 | SafeGuard Encrypted Virtual Disk | vol, hdr |  |
| 3099 | linguist/338 | Sage | .sage, .sagews | text/x-python |
| 3100 | linguist/339 | SaltStack | .sls | text/x-yaml |
| 3101 | fmt/1560 | Sample Vision Audio File Format | smp |  |
| 3102 | fmt/1956 | Sandboxels Save File | sbxls |  |
| 3103 | x-fmt/354 | SAP Document | ali |  |
| 3104 | linguist/328 | SAS | .sas | text/x-sas |
| 3105 | x-fmt/355 | SAS Data File | ssd |  |
| 3106 | x-fmt/192 | SAS for MS-DOS Catalog | sct |  |
| 3107 | x-fmt/356 | SAS for MS-DOS Database | ssd |  |
| 3108 | linguist/340 | Sass | .sass | text/x-sass |
| 3109 | linguist/341 | Scala | .scala, .kojo, .sbt, .sc | text/x-scala |
| 3110 | fmt/91 | Scalable Vector Graphics | svg | image/svg+xml |
| 3111 | fmt/92 | Scalable Vector Graphics | svg | image/svg+xml |
| 3112 | x-fmt/109 | Scalable Vector Graphics Compressed | svgz | image/svg+xml |
| 3113 | fmt/413 | Scalable Vector Graphics Tiny | svg |  |
| 3114 | linguist/342 | Scaml | .scaml |  |
| 3115 | fmt/213 | ScanIt Document | sid |  |
| 3116 | x-fmt/357 | Scanstudio 16-Colour Bitmap | adc |  |
| 3117 | linguist/619814037 | Scenic | .scenic |  |
| 3118 | x-fmt/99 | Schedule+ Contacts | scd |  |
| 3119 | linguist/343 | Scheme | .scm, .sch, .sld, .sls, .sps, .ss | text/x-scheme |
| 3120 | linguist/344 | Scilab | .sci, .sce, .tst |  |
| 3121 | x-fmt/146 | Scitex Continuous Tone Bitmap | ct, sct |  |
| 3122 | fmt/717 | Scream Tracker Module | stm |  |
| 3123 | fmt/718 | Scream Tracker Module | s3m |  |
| 3124 | fmt/1091 | Scribus Document | sla, scd | application/vnd.scribus |
| 3125 | fmt/826 | Scriptware Script Format | sw3 |  |
| 3126 | linguist/329 | SCSS | .scss | text/x-scss |
| 3127 | x-fmt/189 | SDSC Image Tool Run-Length Encoded Bitmap | rle |  |
| 3128 | x-fmt/188 | SDSC Image Tool Wavefront Raster Image | rla |  |
| 3129 | x-fmt/209 | SDSC Image Tool X Window Dump Format | xwd |  |
| 3130 | fmt/1104 | Seattle FilmWorks SFW Image Format | sfw |  |
| 3131 | fmt/318 | Secure DjVU | djvu, djv | image/vnd.djvu, image/x-djvu |
| 3132 | linguist/847830017 | sed | .sed |  |
| 3133 | fmt/363 | SEG Y Data Exchange Format | segy |  |
| 3134 | linguist/345 | Self | .self |  |
| 3135 | fmt/1558 | SelF-eXtracting LHA/LZH Compressed Files | sfx |  |
| 3136 | linguist/880010326 | SELinux Policy | .te |  |
| 3137 | fmt/1553 | Septentrio Binary Format | sbf |  |
| 3138 | fmt/1527 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3139 | fmt/1520 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3140 | fmt/1528 | Serif DrawPlus Drawing | dpp, dpa |  |
| 3141 | fmt/1523 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3142 | fmt/1526 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3143 | fmt/827 | Serif DrawPlus Drawing | dpp |  |
| 3144 | fmt/852 | Serif DrawPlus Drawing | dpp |  |
| 3145 | fmt/1522 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3146 | fmt/1521 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3147 | fmt/1524 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3148 | fmt/853 | Serif DrawPlus Drawing | dpp |  |
| 3149 | fmt/1525 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3150 | fmt/1519 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| 3151 | fmt/1529 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3152 | fmt/681 | Serif PagePlus Publication | ppp |  |
| 3153 | fmt/672 | Serif PagePlus Publication | ppp |  |
| 3154 | fmt/671 | Serif PagePlus Publication | ppp |  |
| 3155 | fmt/1533 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3156 | fmt/1537 | Serif PagePlus Publication | ppp, ppt |  |
| 3157 | fmt/1535 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3158 | fmt/677 | Serif PagePlus Publication | ppp |  |
| 3159 | fmt/675 | Serif PagePlus Publication | ppp |  |
| 3160 | fmt/1531 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3161 | fmt/673 | Serif PagePlus Publication | ppp |  |
| 3162 | fmt/1532 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3163 | fmt/674 | Serif PagePlus Publication | ppp |  |
| 3164 | fmt/676 | Serif PagePlus Publication | ppp |  |
| 3165 | fmt/678 | Serif PagePlus Publication | ppp |  |
| 3166 | fmt/1530 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3167 | fmt/1536 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3168 | fmt/679 | Serif PagePlus Publication | ppp |  |
| 3169 | fmt/680 | Serif PagePlus Publication | ppp |  |
| 3170 | fmt/1534 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| 3171 | fmt/1517 | Serif PhotoPlus Image | spp |  |
| 3172 | fmt/1518 | Serif PhotoPlus Image | spp |  |
| 3173 | fmt/1901 | SGI Movie File | mv, movie |  |
| 3174 | fmt/1618 | SGML/XML Entity File | ent | application/xml-external-parsed-entity |
| 3175 | fmt/992 | SHA1 File | sha1 |  |
| 3176 | fmt/991 | SHA256 File | sha256 |  |
| 3177 | fmt/1797 | SHA512 File | sha512 |  |
| 3178 | linguist/664257356 | ShaderLab | .shader |  |
| 3179 | linguist/346 | Shell | .sh, .bash, .bats, .cgi, .command, .fcgi, .ksh, .sh.in, .tmux, .tool, .trigger, .zsh, .zsh-theme | text/x-sh |
| 3180 | fmt/329 | Shell Archive Format | shar | application/x-sh, application/x-shar |
| 3181 | linguist/687511714 | ShellCheck Config |  | text/x-properties |
| 3182 | linguist/347 | ShellSession | .sh-session | text/x-sh |
| 3183 | linguist/348 | Shen | .shen |  |
| 3184 | fmt/1961 | Shorten (codec) | shn |  |
| 3185 | fmt/995 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| 3186 | fmt/161 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| 3187 | fmt/1196 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| 3188 | fmt/1777 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| 3189 | fmt/696 | Sibelius | sib |  |
| 3190 | fmt/1994 | Sibelius Scorch | sco |  |
| 3191 | fmt/1987 | Sibelius Score | sib | application/x-sibelius-score |
| 3192 | fmt/1984 | Sibelius Score | sib | application/x-sibelius-score |
| 3193 | fmt/1979 | Sibelius Score | sib | application/x-sibelius-score |
| 3194 | fmt/1991 | Sibelius Score | sib | application/x-sibelius-score |
| 3195 | fmt/1985 | Sibelius Score | sib | application/x-sibelius-score |
| 3196 | fmt/1983 | Sibelius Score | sib | application/x-sibelius-score |
| 3197 | fmt/1989 | Sibelius Score | sib | application/x-sibelius-score |
| 3198 | fmt/1980 | Sibelius Score | sib | application/x-sibelius-score |
| 3199 | fmt/1978 | Sibelius Score | sib | application/x-sibelius-score |
| 3200 | fmt/1992 | Sibelius Score | sib | application/x-sibelius-score |
| 3201 | fmt/1988 | Sibelius Score | sib | application/x-sibelius-score |
| 3202 | fmt/1981 | Sibelius Score | sib | application/x-sibelius-score |
| 3203 | fmt/1982 | Sibelius Score | sib | application/x-sibelius-score |
| 3204 | fmt/1990 | Sibelius Score | sib | application/x-sibelius-score |
| 3205 | fmt/1993 | Sibelius Score | sib | application/x-sibelius-score |
| 3206 | fmt/1986 | Sibelius Score | sib | application/x-sibelius-score |
| 3207 | fmt/1229 | Sibelius Sound Set Definition | set |  |
| 3208 | fmt/1148 | SIDOUN WinAVA Format | swa |  |
| 3209 | fmt/883 | Siegfried Signature File | sig |  |
| 3210 | linguist/208976687 | Sieve | .sieve | application/sieve |
| 3211 | fmt/661 | Sigma RAW Image | x3f |  |
| 3212 | x-fmt/358 | Silicon Graphics Graphics File |  |  |
| 3213 | x-fmt/140 | Silicon Graphics Image | bw, rgb | image/x-sgi-bw |
| 3214 | x-fmt/186 | Silicon Graphics RGB File |  |  |
| 3215 | fmt/1067 | Silo | silo |  |
| 3216 | fmt/1068 | Silo | silo |  |
| 3217 | linguist/735623761 | Simple File Verification | .sfv | text/x-properties |
| 3218 | fmt/933 | Simple Vector Format | svf | image/vnd-svf |
| 3219 | fmt/934 | Simple Vector Format | svf | image/vnd-svf |
| 3220 | linguist/987024632 | Singularity |  |  |
| 3221 | fmt/1230 | SK-XML | ddoc |  |
| 3222 | fmt/1271 | SketchUp Document | skp, skb |  |
| 3223 | fmt/1263 | SketchUp Document | skp, skb |  |
| 3224 | x-fmt/452 | SketchUp Document |  |  |
| 3225 | fmt/1269 | SketchUp Document | skp, skb |  |
| 3226 | fmt/1270 | SketchUp Document | skp, skb |  |
| 3227 | fmt/1260 | SketchUp Document | skp, skb |  |
| 3228 | fmt/1268 | SketchUp Document | skp, skb |  |
| 3229 | fmt/1266 | SketchUp Document | skp, skb |  |
| 3230 | fmt/1273 | SketchUp Document | skp, skb |  |
| 3231 | fmt/1265 | SketchUp Document | skp, skb |  |
| 3232 | fmt/1272 | SketchUp Document | skp, skb |  |
| 3233 | fmt/1264 | SketchUp Document | skp, skb |  |
| 3234 | fmt/1261 | SketchUp Document | skp, skb |  |
| 3235 | fmt/1262 | SketchUp Document | skp, skb |  |
| 3236 | x-fmt/451 | SketchUp Document | skb, skp |  |
| 3237 | fmt/1267 | SketchUp Document | skp, skb |  |
| 3238 | fmt/1259 | SketchUp Document | skb, skp |  |
| 3239 | linguist/349 | Slash | .sl |  |
| 3240 | linguist/894641667 | Slice | .ice |  |
| 3241 | linguist/350 | Slim | .slim | text/x-slim |
| 3242 | linguist/119900149 | Slint | .slint |  |
| 3243 | fmt/1234 | Smacker Video | smk | video/vnd.radgamettools.smacker |
| 3244 | linguist/351 | Smali | .smali |  |
| 3245 | linguist/352 | Smalltalk | .st, .cs | text/x-stsrc |
| 3246 | fmt/623 | SmartDraw | sdr |  |
| 3247 | linguist/353 | Smarty | .tpl | text/x-smarty |
| 3248 | linguist/1027892786 | Smithy | .smithy | text/x-csrc |
| 3249 | linguist/164123055 | SmPL | .cocci |  |
| 3250 | linguist/330 | SMT | .smt2, .smt |  |
| 3251 | linguist/151241392 | Snakemake | .smk, .snakefile | text/x-python |
| 3252 | fmt/1057 | SNAP Archive Data File | adf |  |
| 3253 | fmt/1056 | SNAP Main Data File | mdf |  |
| 3254 | fmt/1058 | SNAP Processed Data File | snpdf |  |
| 3255 | fmt/781 | Snoop Packet Capture | snoop |  |
| 3256 | fmt/1359 | Softdisk Text Compressor | ctx |  |
| 3257 | fmt/1167 | Softimage 3D Picture File Format | pic |  |
| 3258 | fmt/1711 | Software602 Printer Configuration File | cfg |  |
| 3259 | linguist/237469032 | Solidity | .sol |  |
| 3260 | fmt/1967 | Solidworks Design Document Files | sldprt, slddrw, sldasm, sld, sldlfp, slddrt |  |
| 3261 | fmt/1962 | SolidWorks Material Database File | sldmat |  |
| 3262 | fmt/951 | Sonic Foundry WAVE 64 | w64, wav |  |
| 3263 | fmt/1274 | Sonic Scenarist Closed Caption Format | scc |  |
| 3264 | fmt/191 | Sony ARW RAW Image File | arw |  |
| 3265 | fmt/1127 | Sony ARW RAW Image File | arw |  |
| 3266 | fmt/472 | Sony Digital Voice File/Sony Memory Stick Voice File | msv, dvf |  |
| 3267 | fmt/1335 | Sony PictureGear Studio Binder | bxu, bxt |  |
| 3268 | fmt/1333 | Sony PictureGear Studio PhotoAlbum | amu, amd |  |
| 3269 | fmt/1334 | Sony PictureGear Studio PrintStudio | lmu, lmd |  |
| 3270 | fmt/1207 | Sony SFK File | sfk |  |
| 3271 | fmt/1764 | Sony SLV File | slv |  |
| 3272 | fmt/1766 | Sony SML File | sml |  |
| 3273 | fmt/1126 | Sony SR2 RAW Image File | sr2 |  |
| 3274 | linguist/222900098 | Soong |  |  |
| 3275 | fmt/1247 | SOSI | sos | text/vnd.sosi |
| 3276 | fmt/1250 | SOSI | sos | text/vnd.sosi |
| 3277 | fmt/1249 | SOSI | sos | text/vnd.sosi |
| 3278 | fmt/1246 | SOSI | sos | text/vnd.sosi |
| 3279 | fmt/1248 | SOSI | sos | text/vnd.sosi |
| 3280 | fmt/209 | Sound Designer II Audio File | sd2 |  |
| 3281 | linguist/354 | SourcePawn | .sp, .inc |  |
| 3282 | fmt/1226 | Sparky | ucsf |  |
| 3283 | linguist/331 | SPARQL | .sparql, .rq | application/sparql-query |
| 3284 | fmt/1575 | Spectrum 512 Compressed | Spectrum 512 Smooshed | spc, sps |  |
| 3285 | fmt/1578 | Spectrum 512 Extended | spx |  |
| 3286 | fmt/1577 | Spectrum 512 Extended | spx |  |
| 3287 | fmt/1576 | Spectrum 512 Uncompressed | Spectrum 512 Uncompressed Enhanced | spu |  |
| 3288 | x-fmt/132 | Speller Custom Dictionary | dic |  |
| 3289 | x-fmt/133 | Speller Exclude Dictionary | dic |  |
| 3290 | fmt/1996 | SPIR-V | spirv |  |
| 3291 | linguist/767169629 | Spline Font Database | .sfd |  |
| 3292 | fmt/1561 | SpritePad Image Format | spd |  |
| 3293 | fmt/638 | SPSS Data File | sav |  |
| 3294 | fmt/274 | SPSS Output File (spv) | spv |  |
| 3295 | fmt/1869 | SPSS PC File Format |  |  |
| 3296 | fmt/997 | SPSS Portable Data Format | por |  |
| 3297 | fmt/1579 | SPYne Containers | spy |  |
| 3298 | linguist/332 | SQF | .sqf, .hqf |  |
| 3299 | linguist/333 | SQL | .sql, .cql, .ddl, .inc, .mysql, .prc, .tab, .udf, .viw | text/x-sql |
| 3300 | fmt/729 | SQLite Database File Format | sqlite, db, db3, sqlite3 | application/x-sqlite3 |
| 3301 | fmt/1135 | SQLite Database File Format | sqlite, db |  |
| 3302 | linguist/334 | SQLPL | .sql, .db2 | text/x-sql |
| 3303 | linguist/355 | Squirrel | .nut | text/x-c++src |
| 3304 | linguist/335 | SRecode Template | .srt | text/x-common-lisp |
| 3305 | linguist/554920715 | SSH Config |  |  |
| 3306 | fmt/1653 | STAD PAC File | pac, seq |  |
| 3307 | linguist/356 | Stan | .stan |  |
| 3308 | fmt/1555 | Standard Data Format | sdf |  |
| 3309 | fmt/504 | Standard Flowgram Format | sff |  |
| 3310 | fmt/698 | Standard for the Exchange of Product model data | step, stp, p21 |  |
| 3311 | x-fmt/195 | Standard Generalized Markup Language | sgml, sgm | text/sgml |
| 3312 | linguist/357 | Standard ML | .ml, .fun, .sig, .sml | text/x-ocaml |
| 3313 | linguist/424510560 | STAR | .star |  |
| 3314 | linguist/960266174 | Starlark | .bzl, .star | text/x-python |
| 3315 | fmt/1556 | Starlink Data Format | sdf |  |
| 3316 | x-fmt/359 | StarOffice Calc | sdc |  |
| 3317 | fmt/809 | StarOffice Calc | sdc |  |
| 3318 | x-fmt/404 | StarOffice Calc |  |  |
| 3319 | fmt/808 | StarOffice Calc | sdc |  |
| 3320 | x-fmt/402 | StarOffice Draw |  |  |
| 3321 | fmt/811 | StarOffice Draw | sdd |  |
| 3322 | fmt/810 | StarOffice Draw | sdd |  |
| 3323 | x-fmt/401 | StarOffice Draw | sda | application/vnd.stardivision.draw |
| 3324 | fmt/814 | StarOffice Impress | sdd |  |
| 3325 | x-fmt/360 | StarOffice Impress | sdd |  |
| 3326 | fmt/815 | StarOffice Impress | sdd |  |
| 3327 | x-fmt/405 | StarOffice Impress |  |  |
| 3328 | x-fmt/400 | StarOffice Writer | sdw | application/vnd.stardivision.writer |
| 3329 | fmt/813 | StarOffice Writer | sdw |  |
| 3330 | x-fmt/403 | StarOffice Writer |  |  |
| 3331 | fmt/812 | StarOffice Writer | sdw |  |
| 3332 | linguist/358 | Stata | .do, .ado, .doh, .ihlp, .mata, .matah, .sthlp |  |
| 3333 | fmt/1598 | Stata .do Command File | do |  |
| 3334 | fmt/1037 | Stata Data (DTA) Format | dta |  |
| 3335 | fmt/1036 | Stata Data (DTA) Format | dta |  |
| 3336 | fmt/1031 | Stata Data (DTA) Format | dta |  |
| 3337 | fmt/1030 | Stata Data (DTA) Format | dta |  |
| 3338 | fmt/1029 | Stata Data (DTA) Format | dta |  |
| 3339 | fmt/1033 | Stata Data (DTA) Format | dta |  |
| 3340 | fmt/1035 | Stata Data (DTA) Format | dta |  |
| 3341 | fmt/1032 | Stata Data (DTA) Format | dta |  |
| 3342 | fmt/1034 | Stata Data (DTA) Format | dta |  |
| 3343 | x-fmt/361 | StatGraphics Data File | aws |  |
| 3344 | x-fmt/131 | Stationery for Mac OS X | doc |  |
| 3345 | fmt/210 | Statistica Report File | str |  |
| 3346 | fmt/606 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| 3347 | fmt/1026 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| 3348 | fmt/1024 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| 3349 | fmt/1028 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| 3350 | fmt/1023 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| 3351 | fmt/605 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| 3352 | fmt/1027 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| 3353 | fmt/1025 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| 3354 | fmt/602 | Statistical Analysis System Catalogue XPT (Unix) | xpt |  |
| 3355 | fmt/601 | Statistical Analysis System Catalogue XPT (Windows) | xpt |  |
| 3356 | fmt/1022 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| 3357 | fmt/1018 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| 3358 | fmt/1016 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| 3359 | fmt/1020 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| 3360 | fmt/608 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| 3361 | fmt/1015 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| 3362 | fmt/1021 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| 3363 | fmt/1017 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| 3364 | fmt/607 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| 3365 | fmt/1019 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| 3366 | fmt/604 | Statistical Analysis System Data XPT (Unix) | xpt |  |
| 3367 | fmt/603 | Statistical Analysis System Data XPT (Windows) | xpt |  |
| 3368 | x-fmt/145 | Stats+ Data File |  |  |
| 3369 | x-fmt/449 | Steel Detailing Neutral Format | sdn | text/plain |
| 3370 | fmt/113 | Still Picture Interchange File Format |  | image/jpeg |
| 3371 | fmt/112 | Still Picture Interchange File Format | spf, jpg | image/jpeg |
| 3372 | linguist/455361735 | STL | .stl |  |
| 3373 | x-fmt/108 | STL (Standard Tessellation Language) ASCII | stl |  |
| 3374 | fmt/865 | STL (Standard Tessellation Language) Binary | stl |  |
| 3375 | linguist/336 | STON | .ston |  |
| 3376 | fmt/1467 | STOS Memory Bank | mbk |  |
| 3377 | fmt/1204 | Strata StudioPro Vis Format |  |  |
| 3378 | x-fmt/362 | StratGraphics Data File | asf |  |
| 3379 | linguist/89855901 | StringTemplate | .st | text/html |
| 3380 | fmt/245 | Structured Data Exchange Format |  |  |
| 3381 | fmt/206 | Structured Query Language Data | sql |  |
| 3382 | fmt/1402 | Student Writing Center Journal | jn, jnt |  |
| 3383 | fmt/1405 | Student Writing Center Letter | lt, ltt |  |
| 3384 | fmt/1404 | Student Writing Center Newsletter | nl, nlt |  |
| 3385 | fmt/1401 | Student Writing Center Report | rp, rpt |  |
| 3386 | fmt/1403 | Student Writing Center Sign | sg, sgt |  |
| 3387 | fmt/1460 | Stuffit Archive File | sit | application/x-stuffit |
| 3388 | fmt/1459 | Stuffit Archive File | sit | application/x-stuffit |
| 3389 | fmt/639 | Stuffit Archive File | sit | application/x-stuffit |
| 3390 | fmt/399 | Stuffit X Archive File | sitx |  |
| 3391 | linguist/359 | Stylus | .styl |  |
| 3392 | fmt/1218 | SubRip Subtitle File | srt |  |
| 3393 | linguist/360 | SubRip Text | .srt |  |
| 3394 | linguist/826404698 | SugarSS | .sss |  |
| 3395 | x-fmt/184 | Sun Raster Image | ras, sun | image/x-sun-raster |
| 3396 | x-fmt/364 | SuperCalc Spreadsheet | cal |  |
| 3397 | fmt/403 | SuperCalc Spreadsheet | cal |  |
| 3398 | x-fmt/363 | SuperCalc Spreadsheet | cal |  |
| 3399 | linguist/361 | SuperCollider | .sc, .scd |  |
| 3400 | fmt/734 | SuperScape Virtual Reality Format | svr |  |
| 3401 | fmt/1276 | SureThing Project File | std |  |
| 3402 | fmt/1552 | Surprise! Adlib Tracker v2.0 | sa2 |  |
| 3403 | linguist/928734530 | Svelte | .svelte | text/html |
| 3404 | linguist/337 | SVG | .svg | text/xml |
| 3405 | linguist/271471144 | Sway | .sw | text/x-rustsrc |
| 3406 | linguist/558779190 | Sweave | .rnw |  |
| 3407 | linguist/362 | Swift | .swift | text/x-swift |
| 3408 | linguist/1066250075 | SWIG | .i | text/x-c++src |
| 3409 | fmt/1865 | SWiSH Movie File | swi |  |
| 3410 | fmt/1583 | SXG (ZX Spectrum) Graphic File | sxg |  |
| 3411 | fmt/205 | Synchronized Multimedia Integration Language (Generic) | smil, smi |  |
| 3412 | fmt/1178 | Synthetic Music Mobile Application Format | mmf | application/vnd.yamaha.smaf-audio |
| 3413 | linguist/363 | SystemVerilog | .sv, .svh, .vh | text/x-systemverilog |
| 3414 | fmt/820 | T64 Tape Image Format | t64 |  |
| 3415 | x-fmt/13 | Tab-separated Values | tsv, tab | text/tab-separated-values |
| 3416 | linguist/606708469 | Tact | .tact |  |
| 3417 | fmt/8 | Tagged Image File Format |  |  |
| 3418 | fmt/9 | Tagged Image File Format |  |  |
| 3419 | fmt/353 | Tagged Image File Format | tif, tiff | image/tiff |
| 3420 | fmt/7 | Tagged Image File Format |  |  |
| 3421 | fmt/10 | Tagged Image File Format |  |  |
| 3422 | fmt/154 | Tagged Image File Format for Electronic Photography (TIFF/EP) | tif, tiff | image/tiff |
| 3423 | fmt/153 | Tagged Image File Format for Image Technology (TIFF/IT) | tif, tiff | image/tiff |
| 3424 | fmt/156 | Tagged Image File Format for Internet Fax (TIFF-FX) | tif, tiff, tfx | image/tiff |
| 3425 | linguist/959889508 | Talon | .talon |  |
| 3426 | fmt/802 | TAP (Commodore 64) | tap |  |
| 3427 | fmt/801 | TAP (ZX Spectrum) | tap |  |
| 3428 | x-fmt/265 | Tape Archive Format | tar | application/x-tar |
| 3429 | fmt/1589 | Taquart Interlace Picture | tip |  |
| 3430 | linguist/367 | Tcl | .tcl, .adp, .sdc, .tcl.in, .tm, .xdc | text/x-tcl |
| 3431 | fmt/1099 | TCR eBook | tcr |  |
| 3432 | linguist/368 | Tcsh | .tcsh, .csh | text/x-sh |
| 3433 | linguist/370 | Tea | .tea |  |
| 3434 | fmt/1475 | TEI P4 XML - Corpus File | xml, tei, odd | application/tei+xml |
| 3435 | fmt/1474 | TEI P4 XML - Single Text File | xml, tei, odd | application/tei+xml |
| 3436 | fmt/1476 | TEI P5 - Single Text File | xml, tei, odd | application/tei+xml |
| 3437 | fmt/1477 | TEI P5 XML - Corpus File | xml, tei, odd | application/tei+xml |
| 3438 | linguist/795579337 | templ | .templ |  |
| 3439 | linguist/371 | Terra | .t | text/x-lua |
| 3440 | linguist/856832701 | Terraform Template | .tftpl | text/x-ruby |
| 3441 | linguist/369 | TeX | .tex, .aux, .bbx, .cbx, .cls, .dtx, .ins, .lbx, .ltx, .mkii, .mkiv, .mkvi, .sty, .toc | text/x-stex |
| 3442 | x-fmt/365 | TeX Binary File | dvi | application/x-dvi |
| 3443 | fmt/160 | TeX/LaTeX Device Independent Document | dvi | application/x-dvi |
| 3444 | linguist/988020015 | Texinfo | .texinfo, .texi, .txi |  |
| 3445 | default/1 | Text |  | text/plain |
| 3446 | linguist/372 | Text | .txt, .fr, .nb, .ncl, .no |  |
| 3447 | x-fmt/421 | Text Configuration file | ini |  |
| 3448 | apache-httpd/16826517866150772418 | text/cache-manifest | appcache | text/cache-manifest |
| 3449 | apache-httpd/9053165588289940551 | text/javascript | js, mjs | text/javascript |
| 3450 | apache-httpd/18042305944542558408 | text/prs.lines.tag | dsc | text/prs.lines.tag |
| 3451 | apache-httpd/14552983977238544941 | text/richtext | rtx | text/richtext |
| 3452 | apache-httpd/15474808736122015508 | text/troff | t, tr, roff, man, me, ms | text/troff |
| 3453 | apache-httpd/14463557769074081415 | text/uri-list | uri, uris, urls | text/uri-list |
| 3454 | apache-httpd/6202298358182500671 | text/vnd.curl | curl | text/vnd.curl |
| 3455 | apache-httpd/6263140658925115359 | text/vnd.curl.dcurl | dcurl | text/vnd.curl.dcurl |
| 3456 | apache-httpd/77337554741314457 | text/vnd.curl.mcurl | mcurl | text/vnd.curl.mcurl |
| 3457 | apache-httpd/1571811637357680016 | text/vnd.curl.scurl | scurl | text/vnd.curl.scurl |
| 3458 | apache-httpd/14440920598703145864 | text/vnd.dvb.subtitle | sub | text/vnd.dvb.subtitle |
| 3459 | apache-httpd/5721063115553544283 | text/vnd.fly | fly | text/vnd.fly |
| 3460 | apache-httpd/13747210224170367135 | text/vnd.fmi.flexstor | flx | text/vnd.fmi.flexstor |
| 3461 | apache-httpd/15955169309864147200 | text/vnd.graphviz | gv | text/vnd.graphviz |
| 3462 | apache-httpd/7954558507145459927 | text/vnd.in3d.3dml | 3dml | text/vnd.in3d.3dml |
| 3463 | apache-httpd/14440943984996457076 | text/vnd.in3d.spot | spot | text/vnd.in3d.spot |
| 3464 | apache-httpd/11118749754585272774 | text/vnd.sun.j2me.app-descriptor | jad | text/vnd.sun.j2me.app-descriptor |
| 3465 | apache-httpd/5953949104867887994 | text/vnd.wap.wmlscript | wmls | text/vnd.wap.wmlscript |
| 3466 | apache-httpd/111619794799683331 | text/x-asm | s, asm | text/x-asm |
| 3467 | apache-httpd/15264676786415624429 | text/x-c | c, cc, cxx, cpp, h, hh, dic | text/x-c |
| 3468 | apache-httpd/14275005348846661619 | text/x-fortran | f, for, f77, f90 | text/x-fortran |
| 3469 | apache-httpd/774788408177624151 | text/x-java-source | java | text/x-java-source |
| 3470 | apache-httpd/17070028980895976996 | text/x-nfo | nfo | text/x-nfo |
| 3471 | apache-httpd/18287376524010136670 | text/x-opml | opml | text/x-opml |
| 3472 | apache-httpd/4555007942525437763 | text/x-pascal | p, pas | text/x-pascal |
| 3473 | apache-httpd/13062495241578132449 | text/x-setext | etx | text/x-setext |
| 3474 | apache-httpd/17986581962061889359 | text/x-sfv | sfv | text/x-sfv |
| 3475 | apache-httpd/13440853329793760916 | text/x-uuencode | uu | text/x-uuencode |
| 3476 | apache-httpd/3360421230686974468 | text/x-vcalendar | vcs | text/x-vcalendar |
| 3477 | apache-httpd/2856549286746833970 | text/x-vcard | vcf | text/x-vcard |
| 3478 | linguist/965696054 | TextGrid | .TextGrid |  |
| 3479 | linguist/373 | Textile | .textile | text/x-textile |
| 3480 | linguist/981795023 | TextMate Properties |  | text/x-properties |
| 3481 | fmt/1588 | TGIF File Format | tgif, obj |  |
| 3482 | fmt/1094 | The Neuroimaging Informatics Technology Initiative File Format | nii |  |
| 3483 | fmt/798 | The Neuroimaging Informatics Technology Initiative File Format | nii |  |
| 3484 | fmt/1301 | The Print Shop Project | psproj |  |
| 3485 | fmt/1783 | The Spectral Geologist Dataset | tsg |  |
| 3486 | fmt/1782 | The Spectral Geologist Dataset | tsg |  |
| 3487 | fmt/1586 | TheDraw Save File | td |  |
| 3488 | linguist/374 | Thrift | .thrift |  |
| 3489 | fmt/682 | Thumbs DB file | db | application/vnd.microsoft.windows.thumbnail-cache |
| 3490 | linguist/422 | TI Program | .8xp, .8xp.txt |  |
| 3491 | fmt/1909 | TibetDoc Word Document | dct |  |
| 3492 | fmt/1717 | Time Stamp Token | tst | application/vnd.etsi.timestamp-token |
| 3493 | fmt/1487 | Timeline Maker Document | tlm, tlm3, tlm4, tlmp |  |
| 3494 | linguist/118656070 | TL-Verilog | .tlv |  |
| 3495 | linguist/364 | TLA | .tla |  |
| 3496 | linguist/356554395 | Toit | .toit |  |
| 3497 | linguist/365 | TOML | .toml | text/x-toml |
| 3498 | fmt/1802 | Transcriber AG TAG Format | tag |  |
| 3499 | fmt/1803 | Transcriber TRS Format | trs |  |
| 3500 | fmt/496 | TransXchange File Format | txc |  |
| 3501 | fmt/1848 | Trelby Document File | trelby |  |
| 3502 | fmt/1085 | TRIM Context Reference File | tr5, txt |  |
| 3503 | fmt/953 | True Audio | tta | audio/tta |
| 3504 | fmt/952 | True Audio | tta | audio/tta |
| 3505 | fmt/1607 | True Colour Picture [Spooky Sprites] (Atari Falcon) | trp, tru |  |
| 3506 | fmt/1605 | True Colour Sprites [Spooky Sprites] (Atari Falcon) | trs |  |
| 3507 | x-fmt/453 | TrueType Font | ttf | font/ttf |
| 3508 | x-fmt/367 | Truevision TGA Bitmap | tga, icb, vda, vst, afi, bpx |  |
| 3509 | fmt/402 | Truevision TGA Bitmap | tga, icb, vda, vst |  |
| 3510 | linguist/89289301 | TSPLIB data | .tsp |  |
| 3511 | linguist/918334941 | TSQL | .sql |  |
| 3512 | linguist/1035892117 | TSV | .tsv, .vcf |  |
| 3513 | linguist/94901924 | TSX | .tsx | text/jsx |
| 3514 | fmt/1603 | TUNDRA | tnd |  |
| 3515 | x-fmt/199 | Turbo Debugger Keystroke Recording File | tdk |  |
| 3516 | fmt/1585 | TurboCalc Document | tcd |  |
| 3517 | linguist/375 | Turing | .t, .tu |  |
| 3518 | fmt/874 | Turtle | ttl | text/turtle |
| 3519 | linguist/376 | Turtle | .ttl | text/turtle |
| 3520 | fmt/1311 | Tweet JSON | json | application/json |
| 3521 | linguist/377 | Twig | .twig | text/x-twig |
| 3522 | linguist/366 | TXL | .txl |  |
| 3523 | linguist/632765617 | Type Language | .tl |  |
| 3524 | fmt/1601 | Type Library | tlb |  |
| 3525 | fmt/1602 | Type Library | tlb |  |
| 3526 | linguist/378 | TypeScript | .ts, .cts, .mts | application/typescript |
| 3527 | fmt/1652 | Typescript | ts, tsx |  |
| 3528 | linguist/952272597 | TypeSpec | .tsp |  |
| 3529 | linguist/704730682 | Typst | .typ |  |
| 3530 | fmt/1000 | TZX Format | tzx |  |
| 3531 | fmt/1738 | UDF Disc Image | toast, iso, cdr, dmg |  |
| 3532 | fmt/1739 | UDF-ISO 9660 Bridge Disc | toast, iso, cdr, dmg |  |
| 3533 | x-fmt/16 | Unicode Text File |  | text/plain |
| 3534 | fmt/792 | Unified Emulator Format | uef, hq.uef |  |
| 3535 | linguist/379 | Unified Parallel C | .upc | text/x-csrc |
| 3536 | fmt/1478 | Unisig |  |  |
| 3537 | x-fmt/193 | Unisys (Sperry) System Data File | sdf |  |
| 3538 | linguist/380 | Unity3D Asset | .anim, .asset, .mask, .mat, .meta, .prefab, .unity | text/x-yaml |
| 3539 | fmt/702 | Universal 3D File Format | u3d |  |
| 3540 | fmt/1905 | Universal Scene Description ASCII File | usda |  |
| 3541 | linguist/120 | Unix Assembly | .s, .ms |  |
| 3542 | linguist/381 | Uno | .uno | text/x-csharp |
| 3543 | linguist/382 | UnrealScript | .uc | text/x-java |
| 3544 | linguist/383 | UrWeb | .ur, .urs |  |
| 3545 | fmt/1102 | Uuencoded File | uue |  |
| 3546 | linguist/603371597 | V | .v | text/x-go |
| 3547 | fmt/1364 | V-Ray Material | vismat |  |
| 3548 | linguist/386 | Vala | .vala, .vapi |  |
| 3549 | linguist/544060961 | Valve Data Format | .vdf |  |
| 3550 | fmt/985 | Valve Texture Format | vtf | image/vnd.valve.source.texture |
| 3551 | fmt/1122 | VAMAS Surface Chemical Analysis Standard Data Transfer Format | vms |  |
| 3552 | fmt/908 | Variant Call Format | vcf |  |
| 3553 | fmt/907 | Variant Call Format | vcf |  |
| 3554 | fmt/905 | Variant Call Format | vcf |  |
| 3555 | fmt/906 | Variant Call Format | vcf |  |
| 3556 | fmt/1381 | VariCAD Drawing | dwb |  |
| 3557 | linguist/399230729 | VBA | .bas, .cls, .frm, .vba | text/x-vb |
| 3558 | fmt/1906 | VBM (VDC BitMap) File | vbm |  |
| 3559 | linguist/408016005 | VBScript | .vbs | text/vbscript |
| 3560 | fmt/1089 | VBScript (VBS) File | vbs |  |
| 3561 | fmt/387 | VCalendar format | vcs | text/x-vCalendar |
| 3562 | fmt/395 | vCard | vcf, vcard | text/vcard |
| 3563 | fmt/1881 | vCard | vcf, vcard | text/vcard |
| 3564 | linguist/851476558 | vCard | .vcf | text/x-properties |
| 3565 | fmt/1880 | vCard | vcf, vcard | text/vcard |
| 3566 | fmt/1879 | vCard | vcf, vcard | text/vcard |
| 3567 | linguist/384 | VCL | .vcl |  |
| 3568 | fmt/583 | Vector Markup Language | vml, html, htm |  |
| 3569 | fmt/1140 | VectorWorks | vwx | application/vnd.vectorworks |
| 3570 | fmt/686 | Vectorworks | vwx | application/vnd.vectorworks |
| 3571 | fmt/451 | VectorWorks | vwx | application/vnd.vectorworks |
| 3572 | fmt/1139 | VectorWorks | vwx | application/vnd.vectorworks |
| 3573 | fmt/684 | Vectorworks | vwx | application/vnd.vectorworks |
| 3574 | fmt/450 | VectorWorks | vwx | application/vnd.vectorworks |
| 3575 | fmt/1141 | VectorWorks | vwx | application/vnd.vectorworks |
| 3576 | fmt/1142 | VectorWorks Plugin or Script | vso, vst, vsm | application/vnd.vectorworks |
| 3577 | linguist/292377326 | Velocity Template Language | .vtl | text/velocity |
| 3578 | x-fmt/156 | Ventura Publisher | gen |  |
| 3579 | x-fmt/57 | Ventura Publisher Vector Graphics | gem |  |
| 3580 | linguist/387 | Verilog | .v, .veo | text/x-verilog |
| 3581 | fmt/457 | Verity Collection Document Dataset Descriptor Style Set | ddd |  |
| 3582 | fmt/458 | Verity Collection Document Index Descriptor Style Set | did |  |
| 3583 | fmt/454 | Verity Collection Index About File | abt |  |
| 3584 | fmt/461 | Verity Collection Index Descriptor File | wld, ddd, did, pdd |  |
| 3585 | fmt/455 | Verity Collection Index Pending Transaction File | trn |  |
| 3586 | fmt/456 | Verity Collection Index Style Policy | plc |  |
| 3587 | fmt/460 | Verity Collection Partition Definition Descriptor Style Set | pdd |  |
| 3588 | fmt/453 | Verity Collection Stop List | stp |  |
| 3589 | fmt/459 | Verity Collection Word List Descriptor Style Set | wld |  |
| 3590 | linguist/385 | VHDL | .vhdl, .vhd, .vhf, .vhi, .vho, .vhs, .vht, .vhw | text/x-vhdl |
| 3591 | fmt/1610 | Viacom New Media Graphics | vnm, 000 |  |
| 3592 | fmt/383 | VICAR (Video Image Communication and Retrieval) Planetary File Format | img, vic, vicar |  |
| 3593 | fmt/425 | Video Object File (MPEG-2 subset) | vob |  |
| 3594 | apache-httpd/11201281216374005716 | video/3gpp2 | 3g2 | video/3gpp2 |
| 3595 | apache-httpd/12074909978442782798 | video/h261 | h261 | video/h261 |
| 3596 | apache-httpd/13685448499458587184 | video/h263 | h263 | video/h263 |
| 3597 | apache-httpd/5268943525352200654 | video/h264 | h264 | video/h264 |
| 3598 | apache-httpd/7742985520819096109 | video/jpeg | jpgv | video/jpeg |
| 3599 | apache-httpd/7382763164379148745 | video/jpm | jpm, jpgm | video/jpm |
| 3600 | apache-httpd/13859451415498689483 | video/mp2t | ts, m2t, m2ts, mts | video/mp2t |
| 3601 | apache-httpd/2026916056792780507 | video/vnd.dece.hd | uvh, uvvh | video/vnd.dece.hd |
| 3602 | apache-httpd/7744082576199857694 | video/vnd.dece.mobile | uvm, uvvm | video/vnd.dece.mobile |
| 3603 | apache-httpd/4607076044858037958 | video/vnd.dece.pd | uvp, uvvp | video/vnd.dece.pd |
| 3604 | apache-httpd/4112863029613538263 | video/vnd.dece.sd | uvs, uvvs | video/vnd.dece.sd |
| 3605 | apache-httpd/9808524822072493886 | video/vnd.dece.video | uvv, uvvv | video/vnd.dece.video |
| 3606 | apache-httpd/8181634439404576904 | video/vnd.dvb.file | dvb | video/vnd.dvb.file |
| 3607 | apache-httpd/9874601596022162347 | video/vnd.fvt | fvt | video/vnd.fvt |
| 3608 | apache-httpd/15681581293980520132 | video/vnd.mpegurl | mxu, m4u | video/vnd.mpegurl |
| 3609 | apache-httpd/3237304037782105496 | video/vnd.ms-playready.media.pyv | pyv | video/vnd.ms-playready.media.pyv |
| 3610 | apache-httpd/13221059779387609093 | video/vnd.uvvu.mp4 | uvu, uvvu | video/vnd.uvvu.mp4 |
| 3611 | apache-httpd/1757511704341130711 | video/vnd.vivo | viv | video/vnd.vivo |
| 3612 | apache-httpd/1451447924574456576 | video/x-f4v | f4v | video/x-f4v |
| 3613 | apache-httpd/7213489331355052709 | video/x-fli | fli | video/x-fli |
| 3614 | apache-httpd/6948131469390528991 | video/x-m4v | m4v | video/x-m4v |
| 3615 | apache-httpd/4395470199143842936 | video/x-matroska | mkv, mk3d, mks | video/x-matroska |
| 3616 | apache-httpd/16549734338554005100 | video/x-ms-asf | asf, asx | video/x-ms-asf |
| 3617 | apache-httpd/663086526965138497 | video/x-ms-vob | vob | video/x-ms-vob |
| 3618 | apache-httpd/10545300983186267140 | video/x-ms-wm | wm | video/x-ms-wm |
| 3619 | apache-httpd/17481297107178477904 | video/x-ms-wmx | wmx | video/x-ms-wmx |
| 3620 | apache-httpd/10712924823320355462 | video/x-ms-wvx | wvx | video/x-ms-wvx |
| 3621 | apache-httpd/2280517777109703612 | video/x-sgi-movie | movie | video/x-sgi-movie |
| 3622 | apache-httpd/16034155539244253487 | video/x-smv | smv | video/x-smv |
| 3623 | linguist/508563686 | Vim Help File | .txt |  |
| 3624 | linguist/388 | Vim Script | .vim, .vba, .vimrc, .vmb |  |
| 3625 | linguist/81265970 | Vim Snippet | .snip, .snippet, .snippets |  |
| 3626 | fmt/1582 | Vim SWAP File | swp |  |
| 3627 | fmt/1811 | Vips Image | v, vips |  |
| 3628 | fmt/1208 | Virtools File Format | cmo, nmo, vmo, nms |  |
| 3629 | fmt/726 | Virtual Disk Image | vdi |  |
| 3630 | fmt/1356 | Virtual Format (Raster) | vrt |  |
| 3631 | fmt/1357 | Virtual Format (Vector) | vrt |  |
| 3632 | fmt/94 | Virtual Reality Modeling Language | wrl | model/vrml |
| 3633 | fmt/93 | Virtual Reality Modeling Language | wrl | model/vrml |
| 3634 | x-fmt/368 | VisiCalc Database | dif |  |
| 3635 | x-fmt/369 | Vista Pro Graphics | dem |  |
| 3636 | fmt/1088 | Visual Basic (VB) File | vb |  |
| 3637 | linguist/389 | Visual Basic .NET | .vb, .vbhtml | text/x-vb |
| 3638 | linguist/679594952 | Visual Basic 6.0 | .bas, .cls, .ctl, .Dsr, .frm | text/x-vb |
| 3639 | fmt/1590 | Visual Basic Binary Form File | frx |  |
| 3640 | fmt/1542 | Visual Basic Form File | frm |  |
| 3641 | fmt/1541 | Visual Basic Form File | frm |  |
| 3642 | x-fmt/48 | Visual Basic Macro | dvb |  |
| 3643 | fmt/1573 | Visual Basic Project File | vbp |  |
| 3644 | fmt/1574 | Visual Basic Project Workspace File | vbw |  |
| 3645 | fmt/1548 | Visual Basics MAK File | mak |  |
| 3646 | x-fmt/150 | Visual FoxPro Database Container File | dcx |  |
| 3647 | fmt/499 | VivoActive | viv | video/vnd-vivo |
| 3648 | fmt/721 | VLW Font File | vlw |  |
| 3649 | linguist/390 | Volt | .volt | text/x-d |
| 3650 | linguist/391 | Vue | .vue |  |
| 3651 | linguist/1055641948 | Vyper | .vy |  |
| 3652 | fmt/1840 | WACZ | wacz | application/x-wacz |
| 3653 | fmt/1281 | WARC | warc | application/warc |
| 3654 | fmt/1355 | WARC | warc | application/warc |
| 3655 | fmt/289 | WARC | warc | application/warc |
| 3656 | fmt/6 | Waveform Audio | wav | audio/x-wav |
| 3657 | fmt/141 | Waveform Audio (PCMWAVEFORMAT) | wav, wave | audio/x-wav |
| 3658 | fmt/142 | Waveform Audio (WAVEFORMATEX) | wav, wave | audio/x-wav |
| 3659 | fmt/143 | Waveform Audio (WAVEFORMATEXTENSIBLE) | wav, wave | audio/x-wav |
| 3660 | linguist/392 | Wavefront Material | .mtl |  |
| 3661 | fmt/1211 | Wavefront Material Template Library | mtl |  |
| 3662 | fmt/1210 | Wavefront OBJ File | obj |  |
| 3663 | linguist/393 | Wavefront Object | .obj |  |
| 3664 | linguist/374521672 | WDL | .wdl |  |
| 3665 | linguist/394 | Web Ontology Language | .owl |  |
| 3666 | fmt/616 | Web Open Font Format | woff | font/woff |
| 3667 | fmt/1172 | Web Open Font Format | woff2 | font/woff2 |
| 3668 | fmt/1454 | Web Video Text Tracks (WebVTT) Format | vtt | text/vtt |
| 3669 | linguist/956556503 | WebAssembly | .wast, .wat | text/x-common-lisp |
| 3670 | linguist/134534086 | WebAssembly Interface Type | .wit | text/x-webidl |
| 3671 | linguist/395 | WebIDL | .webidl | text/x-webidl |
| 3672 | fmt/573 | WebM | webm | video/webm |
| 3673 | fmt/566 | WebP | webp | image/webp |
| 3674 | fmt/567 | WebP | webp | image/webp |
| 3675 | fmt/568 | WebP | webp | image/webp |
| 3676 | linguist/658679714 | WebVTT | .vtt |  |
| 3677 | linguist/668457123 | Wget Config |  |  |
| 3678 | linguist/836605993 | WGSL | .wgsl |  |
| 3679 | linguist/888779559 | Whiley | .whiley |  |
| 3680 | linguist/228 | Wikitext | .mediawiki, .wiki, .wikitext |  |
| 3681 | linguist/950967261 | Win32 Message File | .mc | text/x-properties |
| 3682 | fmt/1255 | Windows Address Book | wab |  |
| 3683 | fmt/114 | Windows Bitmap | ddb, bmp | image/bmp |
| 3684 | fmt/116 | Windows Bitmap | dib, bmp | image/bmp |
| 3685 | fmt/115 | Windows Bitmap | bmp, dib | image/bmp |
| 3686 | fmt/119 | Windows Bitmap | bmp, dib | image/bmp |
| 3687 | fmt/118 | Windows Bitmap | bmp, dib | image/bmp |
| 3688 | fmt/117 | Windows Bitmap | dib, bmp | image/bmp |
| 3689 | x-fmt/414 | Windows Cabinet File | cab | application/vnd.ms-cab-compressed |
| 3690 | fmt/474 | Windows Help File | hlp |  |
| 3691 | fmt/614 | Windows Imaging Format | wim, swm |  |
| 3692 | fmt/1051 | Windows Journal Format | jnt, jtp |  |
| 3693 | fmt/132 | Windows Media Audio | wma, asf | audio/x-ms-wma |
| 3694 | fmt/584 | Windows Media Metafile | wmx, wax, wvx, asx |  |
| 3695 | fmt/589 | Windows Media Playlist | wpl | application/vnd.ms-wpl |
| 3696 | fmt/133 | Windows Media Video | asf, wmv | video/x-ms-wmv |
| 3697 | fmt/441 | Windows Media Video (WVC1) | wmv |  |
| 3698 | x-fmt/119 | Windows Metafile Image | wmf | image/wmf |
| 3699 | x-fmt/410 | Windows New Executable | exe |  |
| 3700 | fmt/900 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| 3701 | fmt/899 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| 3702 | x-fmt/411 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| 3703 | linguist/969674868 | Windows Registry Entries | .reg | text/x-properties |
| 3704 | x-fmt/420 | Windows Setup File | inf | application/inf |
| 3705 | fmt/1995 | WinFax Fax Image | fxr, fxm, fxs |  |
| 3706 | fmt/497 | Wireless Bitmap | wbmp | image/vnd-wap-wbmp |
| 3707 | fmt/1796 | Wireless Markup Language (WML) Document | wml | text/vnd.wap.wml |
| 3708 | linguist/420 | wisp | .wisp | text/x-clojure |
| 3709 | linguist/686821385 | Witcher Script | .ws |  |
| 3710 | linguist/632745969 | Wollok | .wlk |  |
| 3711 | fmt/1723 | Wordcraft Chapter Files | 001 |  |
| 3712 | fmt/949 | WordPerfect | wp4, wpd | application/vnd.wordperfect |
| 3713 | fmt/1424 | WordPerfect Encrypted Document | wp | application/vnd.wordperfect |
| 3714 | fmt/1222 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| 3715 | fmt/1221 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| 3716 | fmt/1220 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| 3717 | x-fmt/393 | WordPerfect for MS-DOS Document | wp, wp5, wpd, w50, doc | application/vnd.wordperfect |
| 3718 | x-fmt/394 | WordPerfect for MS-DOS/Windows Document | wp5, wpd, w51, wp, doc | application/vnd.wordperfect |
| 3719 | x-fmt/44 | WordPerfect for MS-DOS/Windows Document | doc, wpd, wp6, wp, w60, w61, w62 | application/vnd.wordperfect |
| 3720 | x-fmt/203 | WordPerfect for Windows Document | w52, wp, wpd, wp5 | application/vnd.wordperfect |
| 3721 | fmt/1042 | WordPerfect Graphics Metafile | wpg |  |
| 3722 | x-fmt/395 | WordPerfect Graphics Metafile | wpg |  |
| 3723 | fmt/1850 | WordPerfect Macro File | wpm, wcm |  |
| 3724 | fmt/1837 | WordPerfect Presentations | shw |  |
| 3725 | x-fmt/43 | Wordperfect Secondary File | doc |  |
| 3726 | x-fmt/42 | Wordperfect Secondary File | doc |  |
| 3727 | fmt/882 | Wordstar 2000 |  |  |
| 3728 | x-fmt/370 | WordStar for MS-DOS Document | ws3, ws |  |
| 3729 | x-fmt/261 | WordStar for MS-DOS Document | ws, ws7 |  |
| 3730 | x-fmt/236 | WordStar for MS-DOS Document | ws |  |
| 3731 | x-fmt/205 | WordStar for MS-DOS Document | ws, ws5 |  |
| 3732 | x-fmt/260 | WordStar for MS-DOS Document | ws, ws4 |  |
| 3733 | x-fmt/237 | WordStar for MS-DOS Document | ws, ws6 |  |
| 3734 | x-fmt/262 | WordStar for Windows Document | ws, wsw |  |
| 3735 | x-fmt/206 | WordStar for Windows Document | wsd, ws, wsw |  |
| 3736 | x-fmt/5 | Works for Macintosh Document |  |  |
| 3737 | linguist/396 | World of Warcraft Addon Data | .toc |  |
| 3738 | fmt/1611 | WRAptor Compressed File | wra, wr3 |  |
| 3739 | linguist/713580619 | Wren | .wren |  |
| 3740 | x-fmt/4 | Write for Windows Document | wri |  |
| 3741 | x-fmt/12 | Write for Windows Document | wri |  |
| 3742 | fmt/799 | WriteNow |  |  |
| 3743 | linguist/782911107 | X BitMap | .xbm | text/x-csrc |
| 3744 | linguist/208700028 | X Font Directory Index |  |  |
| 3745 | linguist/781846279 | X PixMap | .xpm, .pm | text/x-csrc |
| 3746 | apache-httpd/14081221343819745019 | x-conference/x-cooltalk | ice | x-conference/x-cooltalk |
| 3747 | x-fmt/299 | X-Windows Bitmap Image | xbm | image/x-xbitmap |
| 3748 | x-fmt/207 | X-Windows Bitmap Image | xbm | image/x-xbitmap |
| 3749 | x-fmt/208 | X-Windows Pixmap Image | xpm | image/x-xpixmap |
| 3750 | fmt/401 | X-Windows Screen Dump | xwd, xdm |  |
| 3751 | x-fmt/300 | X-Windows Screen Dump File | xdm, xwd | image/x-xwindowdump |
| 3752 | linguist/397 | X10 | .x10 |  |
| 3753 | fmt/580 | X3D | x3d |  |
| 3754 | fmt/579 | X3D | x3d |  |
| 3755 | fmt/581 | X3D | x3d |  |
| 3756 | fmt/582 | X3D | x3d |  |
| 3757 | fmt/805 | XAML Binary Format | xbf |  |
| 3758 | fmt/922 | Xar Image Format | xar |  |
| 3759 | linguist/421 | xBase | .prg, .ch, .prw |  |
| 3760 | fmt/1612 | XBIN (eXtended BIN) | xb |  |
| 3761 | linguist/398 | XC | .xc | text/x-csrc |
| 3762 | linguist/225167241 | XCompose |  |  |
| 3763 | fmt/1380 | xdomea |  |  |
| 3764 | fmt/1376 | xdomea | xml |  |
| 3765 | fmt/1378 | xdomea | xml |  |
| 3766 | fmt/1379 | xdomea | xml |  |
| 3767 | fmt/1374 | xdomea | xml |  |
| 3768 | fmt/1377 | xdomea | xml |  |
| 3769 | fmt/1375 | xdomea | xml |  |
| 3770 | fmt/1813 | xdomea | xml |  |
| 3771 | fmt/1479 | XIFF (Xerox Image File Format) | xif | image/vnd.xiff |
| 3772 | fmt/1480 | XIFF (Xerox Image File Format) | xif | image/vnd.xiff |
| 3773 | fmt/1657 | XIMG (Extended GEM Bit Image) | ximg, img |  |
| 3774 | fmt/1659 | XL-Paint | raw |  |
| 3775 | fmt/1658 | XL-Paint MaX | max, xlp |  |
| 3776 | fmt/1447 | XLD4 (Bitmap Image) | q4 |  |
| 3777 | fmt/1448 | XLD4 (Graphic Data Document) | q4d |  |
| 3778 | fmt/1828 | XLSX Strict OOXML Spreadsheet | xlsx | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| 3779 | linguist/399 | XML | .xml, .adml, .admx, .ant, .axaml, .axml, .builds, .ccproj, .ccxml, .clixml, .cproject, .cscfg, .csdef, .csl, .csproj, .ct, .depproj, .dita, .ditamap, .ditaval, .dll.config, .dotsettings, .filters, .fsproj, .fxml, .glade, .gml, .gmx, .grxml, .gst, .hzp, .iml, .ivy, .jelly, .jsproj, .kml, .launch, .mdpolicy, .mjml, .mm, .mod, .mojo, .mxml, .natvis, .ncl, .ndproj, .nproj, .nuspec, .odd, .osm, .pkgproj, .pluginspec, .proj, .props, .ps1xml, .psc1, .pt, .qhelp, .rdf, .res, .resx, .rs, .rss, .sch, .scxml, .sfproj, .shproj, .srdf, .storyboard, .sublime-snippet, .sw, .targets, .tml, .ts, .tsx, .typ, .ui, .urdf, .ux, .vbproj, .vcxproj, .vsixmanifest, .vssettings, .vstemplate, .vxml, .wixproj, .workflow, .wsdl, .wsf, .wxi, .wxl, .wxs, .x3d, .xacro, .xaml, .xib, .xlf, .xliff, .xmi, .xml.dist, .xmp, .xproj, .xsd, .xspec, .xul, .zcml | text/xml |
| 3780 | fmt/1501 | XML Forms Data Format | xfdf | application/vnd.adobe.xfdf |
| 3781 | fmt/979 | XML Property List | plist |  |
| 3782 | linguist/75622871 | XML Property List | .plist, .stTheme, .tmCommand, .tmLanguage, .tmPreferences, .tmSnippet, .tmTheme | text/xml |
| 3783 | x-fmt/280 | XML Schema Definition | xsd | application/xml |
| 3784 | fmt/1613 | XML Shareable Playlist Format | xspf |  |
| 3785 | linguist/405 | Xojo | .xojo_code, .xojo_menu, .xojo_report, .xojo_script, .xojo_toolbar, .xojo_window |  |
| 3786 | linguist/614078284 | Xonsh | .xsh | text/x-python |
| 3787 | linguist/400 | XPages | .xsp-config, .xsp.metadata | text/xml |
| 3788 | linguist/401 | XProc | .xpl, .xproc | text/xml |
| 3789 | linguist/402 | XQuery | .xquery, .xq, .xql, .xqm, .xqy | application/xquery |
| 3790 | linguist/403 | XS | .xs | text/x-csrc |
| 3791 | linguist/404 | XSLT | .xslt, .xsl | text/xml |
| 3792 | linguist/406 | Xtend | .xtend |  |
| 3793 | fmt/1497 | XV Thumbnail | p7 |  |
| 3794 | x-fmt/211 | XYWrite Document | xy3 |  |
| 3795 | x-fmt/372 | XYWrite Document | xyp |  |
| 3796 | x-fmt/210 | XYWrite Document | xy |  |
| 3797 | x-fmt/373 | XYWrite Document | xy4 |  |
| 3798 | x-fmt/371 | XYWrite for Windows Document | xyw |  |
| 3799 | fmt/1098 | XZ File Format | xz |  |
| 3800 | linguist/409 | Yacc | .y, .yacc, .yy |  |
| 3801 | fmt/1870 | Yamaha PSR Disk Manager File | mng |  |
| 3802 | fmt/1662 | Yamaha TX Wave Audio | txw, w01, w02, w03, w04, w05, w06, w07, w08, w09, w10, w11, w12, w13, w14, w15, w16, w17, w18, w19, w20, w21, w22 |  |
| 3803 | fmt/1661 | Yamaha Wave Audio | s01, u01, f01, w01 |  |
| 3804 | linguist/407 | YAML | .yml, .mir, .reek, .rviz, .sublime-syntax, .syntax, .yaml, .yaml-tmlanguage, .yaml.sed, .yml.mysql | text/x-yaml |
| 3805 | fmt/818 | YAML | yaml, yml |  |
| 3806 | linguist/408 | YANG | .yang |  |
| 3807 | fmt/1663 | YAODL (Yet Another Object Description Language) File | ydl |  |
| 3808 | linguist/805122868 | YARA | .yar, .yara |  |
| 3809 | linguist/378760102 | YASnippet | .yasnippet |  |
| 3810 | fmt/1100 | yEnc Encoded File | yenc |  |
| 3811 | linguist/237469033 | Yul | .yul |  |
| 3812 | fmt/1671 | Z Compressed Data | z |  |
| 3813 | fmt/1627 | Z Print Build File | zbd |  |
| 3814 | linguist/952972794 | ZAP | .zap, .xzap |  |
| 3815 | fmt/1673 | ZBrush MatCap | zmt |  |
| 3816 | linguist/40 | Zeek | .zeek, .bro |  |
| 3817 | linguist/494938890 | ZenScript | .zs |  |
| 3818 | linguist/410 | Zephir | .zep |  |
| 3819 | fmt/1242 | ZFO (Form) File | zfo | application/vnd.software602.filler.form-xml-zip |
| 3820 | fmt/1243 | ZFO (Message) File | zfo | application/vnd.software602.filler.form-xml-zip |
| 3821 | fmt/1245 | ZFO (Proof of Delivery) File | zfo | application/vnd.software602.filler.form-xml-zip |
| 3822 | fmt/1244 | ZFO (Sent Message) File | zfo | application/vnd.software602.filler.form-xml-zip |
| 3823 | linguist/646424281 | Zig | .zig, .zig.zon |  |
| 3824 | linguist/973483626 | ZIL | .zil, .mud |  |
| 3825 | linguist/411 | Zimpl | .zimpl, .zmpl, .zpl |  |
| 3826 | x-fmt/263 | ZIP Format | zip | application/zip |
| 3827 | fmt/1143 | ZISRAW (CZI) File Format | czi |  |
| 3828 | fmt/1194 | ZModeler Z3D | z3d |  |
| 3829 | fmt/1193 | ZModeler Z3D | z3d |  |
| 3830 | fmt/1195 | ZModeler Z3D | z3d |  |
| 3831 | fmt/1213 | Zoner Callisto Metafile | zmf |  |
| 3832 | x-fmt/269 | ZOO Compressed Archive | zoo |  |
| 3833 | fmt/1954 | Zoom Project Settings | hprj |  |
| 3834 | fmt/1953 | Zoom Project Settings | hprj |  |
| 3835 | fmt/1496 | ZoomBrowser Ex Thumbnail Cache | info |  |
| 3836 | fmt/898 | Zoomify Image Format | zif |  |
| 3837 | fmt/756 | Zope Export File | zexp |  |
| 3838 | fmt/1097 | ZPAQ Archive Format | zpaq |  |
| 3839 | fmt/1674 | ZyXEL Voice Format Audio | zvd, zyx, ad2 |  |
<!--FILE_TYPES_END-->

## Safety

These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

AND

The PRONOM definitions are provided by The National Archives (UK) under the
[Open Government Licence](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

<a href="https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/theseus-rs/file-type">
<img
  src="https://img.shields.io/static/v1?label=VSCode%20Development%20Container&logo=visualstudiocode&message=Open&color=orange"
  alt="VSCode Development Container"
/>
</a>
<br/>
<a href="https://github.dev/theseus-rs/file-type">
<img
  src="https://img.shields.io/static/v1?label=GitHub%20Codespaces&logo=github&message=Open&color=orange"
  alt="GitHub Codespaces"
/>
</a>
