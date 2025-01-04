# file_type

[![ci](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/file_type/badge.svg)](https://docs.rs/file_type)
[![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
[![Latest version](https://img.shields.io/crates/v/file_type.svg)](https://crates.io/crates/file_type)
[![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Getting Started

A file type.  The file type is determined by examining the file or bytes against known file
signatures and file extensions.

Signatures, extensions and media type data are provided by:
* [The National Archives PRONOM](https://www.nationalarchives.gov.uk/pronom/)
* [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types)
* [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml)

# Example

Detect a Java class file from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
assert_eq!(file_type.id(), "x-fmt/415");
assert_eq!(file_type.name(), "Java Class File");
assert_eq!(file_type.media_types(), Vec::<String>::new());
assert_eq!(file_type.extensions(), vec!["class"]);
```

Detect text from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"hello, world\n");
assert_eq!(file_type.id(), "default/1");
assert_eq!(file_type.name(), "Text");
assert_eq!(file_type.media_types(), vec!["text/plain"]);
assert_eq!(file_type.extensions(), Vec::<String>::new());
```

## Supported File Types

<!--FILE_TYPES_BEGIN-->
File Types: 3844

| Id | Name | Extensions | Media Types |
| ---- | ---- | ----------- | ---------- |
| linguist/0 | 1C Enterprise | .bsl, .os |  |
| linguist/387204628 | 2-Dimensional Array | .2da |  |
| x-fmt/102 | 3D Studio (DOS) 2D Shape File | shp |  |
| fmt/1830 | 3D Studio (DOS) 2D/3D Loft Object File | lft |  |
| fmt/1831 | 3D Studio (DOS) Project File | prj |  |
| x-fmt/19 | 3D Studio | 3ds |  |
| fmt/864 | 3DM | 3dm |  |
| x-fmt/432 | 3DM | 3dm |  |
| x-fmt/433 | 3DM | 3dm |  |
| x-fmt/434 | 3DM | 3dm |  |
| x-fmt/435 | 3DM | 3dm |  |
| fmt/978 | 3DS Max | max, chr |  |
| apache-httpd/12470378540611442998 | 3ds | 3ds | image/x-3ds |
| fmt/357 | 3GPP Audio/Video File | 3gp, 3gpp | audio/3gpp, video/3gpp |
| apache-httpd/15983373516404118760 | 3gpp pic bw large | plb | application/vnd.3gpp.pic-bw-large |
| apache-httpd/3513807239393968462 | 3gpp pic bw small | psb | application/vnd.3gpp.pic-bw-small |
| apache-httpd/11384243993817082359 | 3gpp pic bw var | pvb | application/vnd.3gpp.pic-bw-var |
| apache-httpd/7654423572304751873 | 3gpp2 tcap | tcap | application/vnd.3gpp2.tcap |
| apache-httpd/11201281216374005716 | 3gpp2 | 3g2 | video/3gpp2 |
| apache-httpd/14648864362426505394 | 3m post it notes | pwn | application/vnd.3m.post-it-notes |
| fmt/1275 | 3M Printscape | psc |  |
| fmt/829 | 3MF 3D Manufacturing Format | 3mf | application/vnd.ms-3mfdocument |
| linguist/577529595 | 4D | .4dm |  |
| fmt/1150 | 4X Movie File | 4xm, 4xa |  |
| fmt/1699 | 602 Graph/Chart File | gc6 |  |
| fmt/1774 | 602 Graph/Chart File | gc6 |  |
| fmt/1695 | 602 Text file | 602 |  |
| fmt/1294 | 602Tab Spreadsheet | wls |  |
| fmt/1293 | 602Text Document | wpd, wpt |  |
| x-fmt/21 | 7-bit ANSI Text | ans | text/plain |
| x-fmt/22 | 7-bit ASCII Text | asc | text/plain |
| apache-httpd/12448632667719045031 | 7z compressed | 7z | application/x-7z-compressed |
| fmt/484 | 7Zip format | 7z |  |
| x-fmt/282 | 8-bit ANSI Text | ans | text/plain |
| x-fmt/283 | 8-bit ASCII Text | asc | text/plain |
| apache-httpd/8181556888019162819 | aac | aac | audio/x-aac |
| fmt/980 | AAE Sidecar Format | aae |  |
| linguist/452681853 | ABAP CDS | .asddls |  |
| linguist/1 | ABAP | .abap |  |
| fmt/891 | AbiWord Document Template | awt |  |
| fmt/890 | AbiWord Document | abw |  |
| apache-httpd/13077937943514110784 | abiword | abw | application/x-abiword |
| fmt/1463 | Ableton Live Set | als |  |
| linguist/429 | ABNF | .abnf |  |
| x-fmt/301 | ACBM Graphics | acb |  |
| fmt/1482 | Access Report Snapshot | snp |  |
| fmt/843 | AccessData Custom Content Image (Encrypted) | ad1, ad2, ad3, ad4, ad5 |  |
| fmt/842 | AccessData Custom Content Image | ad1, ad2, ad3, ad4, ad5 |  |
| apache-httpd/16487376073977232996 | accpac simply aso | aso | application/vnd.accpac.simply.aso |
| apache-httpd/16508893075482541318 | accpac simply imp | imp | application/vnd.accpac.simply.imp |
| apache-httpd/9946846471999063966 | ace compressed | ace | application/x-ace-compressed |
| fmt/452 | Acrobat Catalog Cat File | cat |  |
| x-fmt/427 | Acrobat Language definition file | lng |  |
| fmt/14 | Acrobat PDF 1.0 - Portable Document Format | pdf | application/pdf |
| fmt/15 | Acrobat PDF 1.1 - Portable Document Format | pdf | application/pdf |
| fmt/16 | Acrobat PDF 1.2 - Portable Document Format | pdf | application/pdf |
| fmt/17 | Acrobat PDF 1.3 - Portable Document Format | pdf | application/pdf |
| fmt/18 | Acrobat PDF 1.4 - Portable Document Format | pdf | application/pdf |
| fmt/19 | Acrobat PDF 1.5 - Portable Document Format | pdf | application/pdf |
| fmt/20 | Acrobat PDF 1.6 - Portable Document Format | pdf | application/pdf |
| fmt/276 | Acrobat PDF 1.7 - Portable Document Format | pdf | application/pdf |
| fmt/1910 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/1911 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/1912 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/354 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/476 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/477 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/478 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/479 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/480 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/481 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/95 | Acrobat PDF/A - Portable Document Format | pdf | application/pdf |
| fmt/493 | Acrobat PDF/E - Portable Document Format for Engineering PDF/E-1 | pdf | application/pdf |
| fmt/144 | Acrobat PDF/X - Portable Document Format - Exchange 1:1999 | pdf | application/pdf |
| fmt/145 | Acrobat PDF/X - Portable Document Format - Exchange 1:2001 | pdf | application/pdf |
| fmt/157 | Acrobat PDF/X - Portable Document Format - Exchange 1a:2001 | pdf | application/pdf |
| fmt/146 | Acrobat PDF/X - Portable Document Format - Exchange 1a:2003 | pdf | application/pdf |
| fmt/147 | Acrobat PDF/X - Portable Document Format - Exchange 2:2003 | pdf | application/pdf |
| fmt/158 | Acrobat PDF/X - Portable Document Format - Exchange 3:2002 | pdf | application/pdf |
| fmt/148 | Acrobat PDF/X - Portable Document Format - Exchange 3:2003 | pdf | application/pdf |
| fmt/489 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-4p | pdf | application/pdf |
| fmt/488 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-4 | pdf | application/pdf |
| fmt/490 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5g | pdf | application/pdf |
| fmt/492 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5n | pdf | application/pdf |
| fmt/491 | Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5pg | pdf | application/pdf |
| linguist/10 | ActionScript | .as |  |
| x-fmt/138 | Active Server Page | asp |  |
| fmt/1915 | ActiveMime Object | mso |  |
| fmt/498 | ActiveX License Package file | lpk |  |
| apache-httpd/7113264481099194993 | acucobol | acu | application/vnd.acucobol |
| apache-httpd/11344963921236519991 | acucorp | atc, acutc | application/vnd.acucorp |
| fmt/356 | Adaptive Multi-Rate Audio | amr | audio/amr |
| fmt/954 | Adaptive Multi-Rate Wideband Audio | awb | audio/amr-wb |
| linguist/11 | Ada | .adb, .ada, .ads |  |
| linguist/884614762 | Adblock Filter List | .txt |  |
| fmt/697 | Additive Manufacturing File Format | amf |  |
| x-fmt/217 | Adobe ACD | acd |  |
| fmt/1500 | Adobe Acrobat Forms Data Format | fdf | application/vnd.fdf |
| fmt/796 | Adobe After Effects | aep |  |
| fmt/1859 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| fmt/937 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| fmt/942 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| fmt/943 | Adobe Air | air | application/vnd.adobe.air-application-installer-package+zip |
| fmt/1053 | Adobe Audio Waveform | pek |  |
| fmt/1499 | Adobe Audition Session File | sesx |  |
| fmt/1814 | Adobe Color Book for Windows | acb |  |
| fmt/1815 | Adobe Color Swatch | aco |  |
| fmt/871 | Adobe Content Server Message File | acsm | application/vnd.adobe.adept+xml |
| fmt/505 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/506 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/507 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/757 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/758 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/759 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/760 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/761 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/762 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/763 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/764 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/765 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/766 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/767 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/768 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/769 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/770 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/771 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/772 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/773 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/774 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/775 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/776 | Adobe Flash | swf | application/x-shockwave-flash |
| fmt/526 | Adobe Font List | lst |  |
| linguist/147198098 | Adobe Font Metrics | .afm |  |
| apache-httpd/7116538921685197509 | adobe formscentral fcdt | fcdt | application/vnd.adobe.formscentral.fcdt |
| fmt/190 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/533 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/534 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/535 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/536 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/537 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/538 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| fmt/539 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| x-fmt/302 | Adobe FrameMaker Document | fm | application/vnd.framemaker |
| x-fmt/162 | Adobe FrameMaker Interchange Format | mif | application/vnd.mif |
| apache-httpd/2260535582776616 | adobe fxp | fxp, fxpl | application/vnd.adobe.fxp |
| fmt/1863 | Adobe Illustrator CC 2020 Artwork | ai, ait |  |
| fmt/1864 | Adobe Illustrator CC 2020 Artwork | ai, ait |  |
| fmt/1862 | Adobe Illustrator CC Artwork | ai, ait |  |
| fmt/417 | Adobe Illustrator | ai | application/postscript |
| fmt/418 | Adobe Illustrator | ai | application/postscript |
| fmt/419 | Adobe Illustrator | ai | application/postscript |
| fmt/420 | Adobe Illustrator | ai | application/postscript |
| fmt/421 | Adobe Illustrator | ai | application/postscript |
| fmt/422 | Adobe Illustrator | ai, eps | application/postscript |
| fmt/423 | Adobe Illustrator | ai | application/postscript |
| fmt/557 | Adobe Illustrator | ai, eps | application/postscript |
| fmt/558 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/559 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/560 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/561 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/562 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/563 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/564 | Adobe Illustrator | ai, pdf | application/postscript |
| fmt/565 | Adobe Illustrator | ai, pdf | application/postscript |
| x-fmt/20 | Adobe Illustrator | ai | application/postscript |
| fmt/1191 | Adobe InDesign Book | indb |  |
| fmt/1628 | Adobe InDesign Document | indd, ind |  |
| fmt/1629 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1630 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1631 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1632 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1633 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1634 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1635 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1636 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1637 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1638 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1639 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1640 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1941 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/1942 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/196 | Adobe InDesign Document | indd, ind, indt |  |
| fmt/548 | Adobe InDesign Document | ind, indd, indt |  |
| fmt/549 | Adobe InDesign Document | ind, indd, indt |  |
| fmt/550 | Adobe InDesign Document | ind, indd, indt |  |
| fmt/551 | Adobe InDesign Document | ind, indd, indt |  |
| fmt/552 | Adobe InDesign Document | ind, indd, indt |  |
| x-fmt/450 | Adobe InDesign Document | ind, indd, indt | application/octet-stream |
| fmt/1641 | Adobe InDesign Interchange Document | inx |  |
| fmt/1192 | Adobe InDesign Library | indl |  |
| fmt/1642 | Adobe InDesign Library | indl |  |
| fmt/521 | Adobe Multiple Master Metrics font file | mmm |  |
| x-fmt/167 | Adobe PhotoDeluxe | pdd |  |
| fmt/996 | Adobe Photoshop Large Document Format | psb | image/vnd.adobe.photoshop |
| x-fmt/92 | Adobe Photoshop | psd, pdd | image/vnd.adobe.photoshop |
| fmt/446 | Adobe Portable Document Catalog Index File | pdx |  |
| fmt/447 | Adobe Portable Document Catalog Index File | pdx |  |
| fmt/448 | Adobe Portable Document Catalog Index File | pdx |  |
| fmt/449 | Adobe Portable Document Catalog Index File | pdx |  |
| fmt/509 | Adobe PostScript Font Metrics file | pfm |  |
| fmt/525 | Adobe Printer Font Binary | pfb |  |
| fmt/1816 | Adobe Swatch Exchange | ase |  |
| fmt/1190 | Adobe SWC Package | swc |  |
| fmt/660 | Adobe Type 1 Mac Font File |  |  |
| apache-httpd/16997561686559408896 | adobe xdp xml | xdp | application/vnd.adobe.xdp+xml |
| apache-httpd/2263091937087112692 | adpcm | adp | audio/adpcm |
| fmt/1584 | ADRIFT Text Adventure File | taf |  |
| fmt/1370 | Advanced Disk Catalog | adc |  |
| fmt/844 | Advanced Forensic Format | aff |  |
| fmt/683 | Advanced Function Presentation | afp |  |
| fmt/131 | Advanced Systems Format | asf | application/vnd.ms-asf |
| fmt/840 | ADX Audio Format | adx |  |
| fmt/1620 | Aero Studio Song | aero |  |
| linguist/12 | Agda | .agda |  |
| fmt/1505 | Agisoft Point Cloud | oc3 |  |
| fmt/1502 | Agisoft Project Archive | psz |  |
| fmt/1503 | Agisoft Project File | psx |  |
| fmt/1504 | Agisoft Tiled Model | tls |  |
| fmt/1649 | AGS 4 Data Format | ags |  |
| linguist/2 | AGS Script | .asc, .ash | text/x-c++src |
| apache-httpd/4986673632147286331 | ahead space | ahead | application/vnd.ahead.space |
| fmt/1621 | AHX-Module Format (formerly THX module format) | ahx |  |
| linguist/451700185 | AIDL | .aidl |  |
| apache-httpd/4165121783115442141 | airzip filesecure azf | azf | application/vnd.airzip.filesecure.azf |
| apache-httpd/9209038509344480468 | airzip filesecure azs | azs | application/vnd.airzip.filesecure.azs |
| fmt/1449 | Aldus FreeHand Drawing |  |  |
| fmt/1450 | Aldus FreeHand Drawing |  |  |
| x-fmt/303 | Aldus Freehand Drawing | fh3 |  |
| x-fmt/304 | Aldus Freehand Drawing | fh4 |  |
| fmt/1092 | Alias Pix Image File | pix, ico |  |
| fmt/1171 | Alias PowerAnimator File |  |  |
| fmt/1093 | Alias Scene Description Language | sdl |  |
| fmt/1170 | Alias Studio Wire File |  |  |
| fmt/1175 | Alias Studio Wire File |  |  |
| linguist/13 | Alloy | .als |  |
| linguist/14 | Alpine Abuild |  | text/x-sh |
| linguist/187772328 | Altium Designer | .OutJob, .PcbDoc, .PrjPCB, .SchDoc |  |
| linguist/658971832 | AL | .al |  |
| apache-httpd/3660531883732386782 | amazon ebook | azw | application/vnd.amazon.ebook |
| fmt/1937 | Amazon Kindle eBook File | azw, azw3, mobi, amr |  |
| apache-httpd/11880520770616751632 | americandynamics acc | acc | application/vnd.americandynamics.acc |
| x-fmt/290 | AMI Draw Vector Image | sdw |  |
| x-fmt/191 | AMI Professional Document | sam | application/vnd.lotus-wordpro |
| apache-httpd/18255916549466049934 | amiga ami | ami | application/vnd.amiga.ami |
| fmt/1361 | Amiga Disk File | adf |  |
| fmt/917 | AmiraMesh | am, amiramesh, hx |  |
| fmt/918 | AmiraMesh | am, amiramesh, hx |  |
| fmt/919 | AmiraMesh | am, amiramesh, hx |  |
| fmt/920 | AmiraMesh | am, amiramesh, hx |  |
| fmt/921 | AmiraMesh | am, amiramesh, hx |  |
| linguist/3 | AMPL | .ampl, .mod |  |
| apache-httpd/17267421299241784478 | andrew inset | ez | application/andrew-inset |
| apache-httpd/11490512282215503801 | android package archive | apk | application/vnd.android.package-archive |
| linguist/389477596 | AngelScript | .as, .angelscript | text/x-c++src |
| fmt/935 | Animated Portable Network Graphics | png, apng | image/vnd.mozilla.apng |
| fmt/1784 | Animatic Film Format | flm |  |
| apache-httpd/596873008912409075 | anser web certificate issue initiation | cii | application/vnd.anser-web-certificate-issue-initiation |
| apache-httpd/8243032856616937357 | anser web funds transfer initiation | fti | application/vnd.anser-web-funds-transfer-initiation |
| linguist/15 | Ant Build System |  | application/xml |
| apache-httpd/858406622269959928 | antix game component | atx | application/vnd.antix.game-component |
| linguist/1067292663 | Antlers | .antlers.html, .antlers.php, .antlers.xml |  |
| linguist/4 | ANTLR | .g4 |  |
| custom/1 | Apache Arrow | arrow | application/vnd.apache.arrow.file |
| custom/2 | Apache Avro | avro | application/vnd.apache.avro.file |
| custom/5 | Apache Parquet | parquet | application/vnd.apache.parquet |
| linguist/16 | ApacheConf | .apacheconf, .vhost |  |
| linguist/17 | Apex | .cls, .trigger | text/x-java |
| linguist/5 | API Blueprint | .apib |  |
| linguist/6 | APL | .apl, .dyalog | text/apl |
| linguist/18 | Apollo Guidance Computer | .agc |  |
| fmt/416 | Apple Core Audio Format | caf |  |
| fmt/625 | Apple Disk Copy Image | dmg, smi, img, image |  |
| fmt/1071 | Apple Disk Image | dmg | application/x-apple-diskimage |
| fmt/482 | Apple iBook format | ibooks | application/x-ibooks+zip |
| fmt/1185 | Apple Icon Image Format | icns |  |
| apache-httpd/1006546886522380100 | apple installer xml | mpkg | application/vnd.apple.installer+xml |
| fmt/1441 | Apple iWork Document | iwa, key, pages, numbers, template |  |
| fmt/646 | Apple iWork Keynote | key |  |
| fmt/1440 | Apple iWork Numbers |  |  |
| fmt/825 | Apple iWork Numbers | numbers |  |
| fmt/1439 | Apple iWork Pages | pages |  |
| fmt/824 | Apple iWork Pages | pages |  |
| fmt/1187 | Apple iWork Template | template |  |
| fmt/596 | Apple Lossless Audio Codec | m4a, mp4 |  |
| apache-httpd/9044968354667619340 | apple mpegurl | m3u8 | application/vnd.apple.mpegurl |
| fmt/1757 | Apple Partition Map - ISO 9660 - UDF Hybrid Disk Image | iso, toast, dmg |  |
| fmt/1740 | Apple Partition Map Disk Image | toast, iso, cdr, dmg, bin, img |  |
| fmt/1741 | Apple Partition Map ISO 9660 Hybrid | toast, iso, cdr |  |
| fmt/797 | Apple ProRes | mov |  |
| fmt/866 | Apple Safari Webarchive | webarchive |  |
| x-fmt/305 | Apple Sound | afc |  |
| fmt/503 | AppleDouble Resource Fork |  | multipart/appledouble |
| fmt/966 | AppleDouble Resource Fork |  | multipart/appledouble |
| linguist/19 | AppleScript | .applescript, .scpt |  |
| fmt/967 | AppleSingle | as | application/applefile |
| fmt/968 | AppleSingle | as | application/applefile |
| fmt/1715 | Applet Effect Factory Config File | data |  |
| fmt/751 | AppleWorks Database | cwk |  |
| fmt/748 | AppleWorks Drawing | cwk |  |
| fmt/752 | AppleWorks Painting | cwk |  |
| fmt/753 | AppleWorks Presentation | cwk |  |
| fmt/750 | AppleWorks Spreadsheet | cwk |  |
| fmt/749 | AppleWorks Word Processor | cwk |  |
| x-fmt/228 | Applixware Bitmap | im |  |
| x-fmt/220 | Applixware Spreadsheet | as |  |
| apache-httpd/9816146314707347696 | applixware | aw | application/applixware |
| fmt/1473 | Archimedes Tracker Module | musx |  |
| fmt/1835 | Archiver Format | a |  |
| fmt/1833 | ArcSoft Album and SlideShow Files for PhotoStudio and PhotoImpression | abm, sld |  |
| fmt/1832 | ArcSoft PhotoStudio File | psf |  |
| linguist/20 | Arc | .arc |  |
| apache-httpd/16752839964839226118 | aristanetworks swi | swi | application/vnd.aristanetworks.swi |
| fmt/610 | ARJ File Format | arj |  |
| fmt/666 | ART image format | art |  |
| fmt/1623 | Art Of Noise | aon |  |
| fmt/1624 | Art Of Noise | aon |  |
| fmt/1660 | Arts & Letters Clip Art Library | yal |  |
| fmt/1458 | Arts & Letters Graphics File | ged |  |
| linguist/22 | AsciiDoc | .asciidoc, .adoc, .asc |  |
| fmt/1592 | ASEG-GDF2 Description File | des |  |
| fmt/1593 | ASEG-GDF2- Data Definition File | dfn |  |
| linguist/124996147 | ASL | .asl, .dsl |  |
| apache-httpd/111619794799683331 | asm | s, asm | text/x-asm |
| linguist/7 | ASN.1 | .asn, .asn1 | text/x-ttcn-asn |
| fmt/1080 | ASP Application Directive File | asax |  |
| fmt/1081 | ASP Control Directive File | ascx |  |
| fmt/1082 | ASP WebService Directive File | asmx |  |
| linguist/564186416 | ASP.NET | .asax, .ascx, .ashx, .asmx, .aspx, .axd | application/x-aspx |
| linguist/23 | AspectJ | .aj |  |
| fmt/368 | ASPRS Lidar Data Exchange Format | las, laz |  |
| fmt/369 | ASPRS Lidar Data Exchange Format | las, laz |  |
| fmt/370 | ASPRS Lidar Data Exchange Format | las, laz |  |
| linguist/24 | Assembly | .asm, .a51, .i, .inc, .nas, .nasm |  |
| fmt/1564 | Associated Signature Container Extended (ASiC-E) | asice, sce | application/vnd.etsi.asic-e+zip |
| fmt/1341 | Associated Signature Container Simple (ASiC-S) | asics, scs | application/vnd.etsi.asic-s+zip |
| fmt/643 | ASTM E57 3D File Format | e57 | model/e57 |
| apache-httpd/6375177958950997398 | astraea software iota | iota | application/vnd.astraea-software.iota |
| linguist/578209015 | Astro | .astro | text/jsx |
| fmt/1622 | Asylum Music Format | amf |  |
| fmt/1693 | Asymetrix Compel Presentation | cpl, art |  |
| fmt/1694 | Asymetrix Compel Presentation | cpl, art |  |
| fmt/1795 | Asymetrix Toolbook File | tbk, sbk |  |
| fmt/470 | Asymetrix Toolbook File | tbk, sbk |  |
| linguist/591605007 | Asymptote | .asy | text/x-kotlin |
| fmt/495 | ATCO-CIF | cif |  |
| apache-httpd/3641799221620537973 | atom xml | atom | application/atom+xml |
| apache-httpd/2088900736017873143 | atomcat xml | atomcat | application/atomcat+xml |
| apache-httpd/13485419110853384641 | atomsvc xml | atomsvc | application/atomsvc+xml |
| fmt/1968 | Atrac Codec File | aea |  |
| linguist/9 | ATS | .dats, .hats, .sats |  |
| fmt/1822 | Audacity Audio Block File | au |  |
| fmt/1823 | Audacity Project File | aup |  |
| fmt/1824 | Audacity Project File | aup |  |
| fmt/1825 | Audacity Project File | aup |  |
| fmt/1826 | Audacity Project File | aup3 |  |
| fmt/1812 | Audio Data Transport Stream | aac, adts | audio/aac, audio/vnd.dlna.adts |
| x-fmt/136 | Audio Interchange File Format (compressed) | aifc | audio/x-aiff |
| fmt/414 | Audio Interchange File Format | aif, aiff |  |
| x-fmt/135 | Audio Interchange File Format |  |  |
| fmt/5 | Audio/Video Interleaved Format | avi | video/x-msvideo |
| apache-httpd/13429870797822209580 | audiograph | aep | application/vnd.audiograph |
| linguist/25 | Augeas | .aug |  |
| apache-httpd/6628203580258632388 | authorware bin | aab, x32, u32, vox | application/x-authorware-bin |
| apache-httpd/6257513896690260698 | authorware map | aam | application/x-authorware-map |
| apache-httpd/7946561646438379956 | authorware seg | aas | application/x-authorware-seg |
| fmt/1939 | Auto FX PhotoGraphic Edges Image File | afx |  |
| x-fmt/98 | AutoCAD ACIS Export File | sat |  |
| x-fmt/26 | AutoCAD Batch Plot File | bp2, bpl |  |
| x-fmt/27 | AutoCAD Batch Plot File | bp3 |  |
| x-fmt/24 | AutoCAD Block Attribute Template | blk |  |
| x-fmt/37 | AutoCAD Colour-Dependant Plot Style Table | ctb |  |
| x-fmt/68 | AutoCAD Compiled Menu | mnc |  |
| x-fmt/103 | AutoCAD Compiled Shape/Font File | shx |  |
| x-fmt/38 | AutoCAD Custom Dictionary | cus |  |
| x-fmt/441 | AutoCAD Database File Locking Information | dwl | application/octet-stream |
| x-fmt/39 | AutoCAD dbConnect Query Set | dbq |  |
| x-fmt/40 | AutoCAD dbConnect Template Set | dbt |  |
| fmt/977 | AutoCAD Design Web Format(DWFx) | dwfx |  |
| x-fmt/49 | AutoCAD Design Web Format | dwf | application/dwf, application/x-dwf, drawing/x-dwf, image/vnd.dwf, image/x-dwf, model/vnd.dwf |
| x-fmt/134 | AutoCAD Device-Independent Binary Plotter File | adi |  |
| fmt/1456 | Autocad DMP File | dmp |  |
| x-fmt/50 | AutoCAD Drawing Standards File | dws |  |
| x-fmt/51 | AutoCAD Drawing Template | dwt |  |
| fmt/1395 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/21 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/22 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/23 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/24 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/25 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/26 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/27 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/28 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/29 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/30 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/31 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/32 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/33 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/34 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/35 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/36 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/434 | AutoCAD Drawing | dwg | image/vnd.dwg |
| fmt/531 | AutoCAD Drawing | dwg | image/vnd.dwg |
| x-fmt/455 | AutoCAD Drawing | dwg | image/vnd.dwg |
| x-fmt/112 | AutoCAD External Database Configuration File | udl |  |
| x-fmt/155 | AutoCAD Film Roll | flm |  |
| x-fmt/54 | AutoCAD Font Mapping Table | fmp |  |
| fmt/1103 | AutoCAD Hatch Pattern | pat |  |
| x-fmt/61 | AutoCAD Landscape Library | lli |  |
| x-fmt/59 | AutoCAD Last Saved Layer State | las |  |
| x-fmt/60 | AutoCAD Linetype Definition File | lin |  |
| x-fmt/70 | AutoCAD Menu Resource File | mnr, mnt |  |
| x-fmt/107 | AutoCAD Named Plot Style Table | stb |  |
| x-fmt/77 | AutoCAD Plot Configuration File | pc2 |  |
| x-fmt/78 | AutoCAD Plot Configuration File | pc3 |  |
| x-fmt/79 | AutoCAD Plot Configuration File | pcp |  |
| x-fmt/100 | AutoCAD Script | scr |  |
| x-fmt/104 | AutoCAD Slide Library | slb |  |
| x-fmt/105 | AutoCAD Slide | sld | application/sld, application/x-sld, image/x-sld |
| x-fmt/71 | AutoCAD Source Menu File | mns |  |
| x-fmt/72 | AutoCAD Template Menu File | mnu |  |
| fmt/1257 | AutoCAD Temporary File | ac$ |  |
| x-fmt/127 | AutoCAD Xref Log | xlg |  |
| fmt/1916 | Autodesk Alias Wire Format |  |  |
| fmt/299 | Autodesk Animator (FlicLib) | fli |  |
| x-fmt/223 | Autodesk Animator CEL File Format | cel |  |
| fmt/298 | Autodesk Animator Pro FLIC | flc |  |
| x-fmt/154 | AutoDesk FLIC Animation | fli |  |
| fmt/1562 | AutoDesk Indexed Point Cloud | pcg |  |
| fmt/1348 | Autodesk Revit Family File | rfa, rft |  |
| fmt/1349 | Autodesk Revit Family File | rfa, rft |  |
| fmt/1351 | Autodesk Revit Family File | rfa, rft |  |
| fmt/1346 | Autodesk Revit File | rvt, rfa, rte, rft |  |
| fmt/1347 | Autodesk Revit Project File | rvt, rte, rft |  |
| fmt/1350 | Autodesk Revit Project File | rvt, rte |  |
| linguist/26 | AutoHotkey | .ahk, .ahkl |  |
| linguist/27 | AutoIt | .au3 |  |
| x-fmt/63 | AutoLISP File | lsp |  |
| x-fmt/69 | AutoLISP Menu Source File | mnl |  |
| fmt/331 | Autorun Configuration File | inf |  |
| fmt/1461 | Autorun Maestro Menu File | mnu |  |
| fmt/1044 | AutoShade Rendering Slide | rnd |  |
| x-fmt/306 | AutoSketch Drawing | skf |  |
| fmt/1054 | AVCHD Clip Information File | cpi, clpi |  |
| fmt/1076 | AVCHD Index File | bdm, bdmv |  |
| fmt/1075 | AVCHD Movie Object File | bdm, bdmv |  |
| fmt/1074 | AVCHD Playlist File | mpl, mpls |  |
| fmt/1077 | AVCHD Thumbnail Index File | tid |  |
| fmt/1330 | Avery DesignPro Document | zdp |  |
| fmt/1331 | Avery DesignPro Document | zdl |  |
| fmt/1329 | Avery Label Pro Document | lpd |  |
| apache-httpd/14184589139419150626 | avif | avif | image/avif |
| linguist/785497837 | Avro IDL | .avdl |  |
| fmt/1179 | Away3D Data Format | awd |  |
| linguist/28 | Awk | .awk, .auk, .gawk, .mawk, .nawk |  |
| fmt/884 | AXD HTTP Handler File | axd |  |
| fmt/1804 | B Source Code File | b |  |
| linguist/96642275 | B4X | .bas | text/x-vb |
| fmt/941 | Back Up File | bak |  |
| linguist/720859680 | Ballerina | .bal |  |
| fmt/1238 | Band Interleaved By Line (BIL) Image Encoding | bil |  |
| fmt/1239 | Band Interleaved By Pixel (BIP) Image Encoding | bip |  |
| fmt/1240 | Band Sequential (BSQ) Image Encoding | bsq |  |
| fmt/885 | BASIC File | bas |  |
| linguist/28923963 | BASIC | .bas |  |
| x-fmt/413 | Batch file (executable) | bat |  |
| linguist/29 | Batchfile | .bat, .cmd |  |
| fmt/1650 | Bayesian Interchange Format File | bif |  |
| apache-httpd/17607384768001508299 | bcpio | bcpio | application/x-bcpio |
| fmt/1340 | BDOC | bdoc | application/vnd.bdoc-1.0 |
| fmt/1342 | BDOC | bdoc, asice | application/vnd.etsi.asic-e+zip |
| fmt/1559 | Beam Software SIFF File | son, vb |  |
| linguist/545626333 | Beef | .bf | text/x-csharp |
| linguist/30 | Befunge | .befunge, .bf |  |
| fmt/1549 | Bentley Microstation Hidden Line File | hln |  |
| fmt/502 | Bentley V8 DGN | dgn |  |
| linguist/121855308 | Berry | .be |  |
| fmt/687 | Better Portable Graphics | bpg |  |
| fmt/1616 | BibTeX Database File | bib |  |
| linguist/982188347 | BibTeX | .bib, .bibtex | text/x-stex |
| linguist/321200902 | Bicep | .bicep, .bicepparam |  |
| fmt/1917 | BigTIFF | tif, tf8, btf |  |
| linguist/1055528081 | Bikeshed | .bs | text/html |
| fmt/1722 | BIM Metadata File | bim |  |
| fmt/208 | Binary File | bin |  |
| fmt/984 | Binary Property List | plist, nib, aae, iMovieProj, ezdraw |  |
| default/2 | Binary |  | application/octet-stream |
| x-fmt/416 | BinHex Binary Text | hqx | application/mac-binhex40 |
| fmt/731 | Bink Video Format | bik |  |
| fmt/732 | Bink Video Format | bik2, bk2 | video/vnd.radgamettools.bink |
| linguist/31 | Bison | .bison |  |
| linguist/32 | BitBake | .bb, .bbappend, .bbclass, .inc |  |
| fmt/1569 | Bitstream Speedo Fonts | spd |  |
| apache-httpd/8096259863664211066 | bittorrent | torrent | application/x-bittorrent |
| fmt/1052 | BKNAS Seismic Data Format | bknas |  |
| linguist/33 | Blade | .blade, .blade.php |  |
| fmt/902 | Blender 3D | blend |  |
| fmt/903 | Blender 3D | blend |  |
| fmt/1182 | Blitz3D File Format | b3d |  |
| linguist/34 | BlitzBasic | .bb, .decls |  |
| linguist/35 | BlitzMax | .bmx |  |
| apache-httpd/13372282205074757825 | blorb | blb, blorb | application/x-blorb |
| apache-httpd/2469893521022682751 | blueice multipass | mpm | application/vnd.blueice.multipass |
| linguist/641580358 | Bluespec BH | .bs | text/x-haskell |
| linguist/36 | Bluespec | .bsv | text/x-systemverilog |
| fmt/904 | Bluetooth Snoop Packet Capture | log |  |
| apache-httpd/5904094350807371736 | bmi | bmi | application/vnd.bmi |
| fmt/1181 | Bodypaint 3D | b3d |  |
| linguist/955017407 | Boogie | .bpl |  |
| linguist/37 | Boo | .boo |  |
| fmt/393 | Borland Reflex flat datafile | rxd |  |
| linguist/330386870 | BQN | .bqn |  |
| linguist/38 | Brainfuck | .b, .bf | text/x-brainfuck |
| linguist/943571030 | BrighterScript | .bs |  |
| linguist/39 | Brightscript | .brs |  |
| fmt/1836 | Brio Query File | bqy |  |
| fmt/518 | Broad Band eBook | lrf |  |
| fmt/1 | Broadcast WAVE | wav | audio/x-wav |
| fmt/2 | Broadcast WAVE | wav | audio/x-wav |
| fmt/527 | Broadcast WAVE | wav | audio/x-wav |
| fmt/703 | Broadcast WAVE | wav | audio/x-wav |
| fmt/704 | Broadcast WAVE | wav | audio/x-wav |
| fmt/705 | Broadcast WAVE | wav | audio/x-wav |
| fmt/706 | Broadcast WAVE | wav | audio/x-wav |
| fmt/707 | Broadcast WAVE | wav | audio/x-wav |
| fmt/708 | Broadcast WAVE | wav | audio/x-wav |
| fmt/709 | Broadcast WAVE | wav, rf64 | audio/x-wav |
| fmt/710 | Broadcast WAVE | wav, rf64 | audio/x-wav |
| fmt/711 | Broadcast WAVE | wav | audio/x-wav |
| fmt/1299 | Broderbund Print Shop Deluxe | pdb, pds, pcb, pdc, pcc, pce, pdg, pdl, pso, pdp, pho, pcp, ppi, pda |  |
| x-fmt/168 | Broderbund Print Shop Deluxe | pcc, pdb, pdc, pda, pdl, pds, pdg |  |
| fmt/1300 | Broderbund The Print Shop/PrintMaster/American Greetings Project | ban, bro, biz, cal, car, cer, env, fax, sig, cft, hcr, lbl, let, nws, not, pcr, php, tsh, web, sti |  |
| linguist/153503348 | Browserslist |  |  |
| fmt/1385 | Bruker PDZ | pdz, xpdz |  |
| fmt/439 | BSDIFF | bsdiff |  |
| x-fmt/308 | Btrieve Database | btr |  |
| apache-httpd/9491002127237071349 | businessobjects | rep | application/vnd.businessobjects |
| x-fmt/267 | BZIP Compressed Archive | bz |  |
| x-fmt/268 | BZIP2 Compressed Archive | bz2 | application/x-bzip2 |
| apache-httpd/18270236438859893354 | bzip | bz | application/x-bzip |
| fmt/1768 | C Source Code File | c |  |
| linguist/42 | C# | .cs, .cake, .cs.pp, .csx, .linq | text/x-csharp |
| fmt/1769 | C++ Source Code File | cpp, cxx, cc |  |
| linguist/43 | C++ | .cpp, .c++, .cc, .cp, .cppm, .cxx, .h, .h++, .hh, .hpp, .hxx, .inc, .inl, .ino, .ipp, .ixx, .re, .tcc, .tpp, .txx | text/x-c++src |
| linguist/44 | C-ObjDump | .c-objdump |  |
| fmt/1735 | C/C++ Header File | h, hpp, hxx |  |
| linguist/45 | C2hs Haskell | .chs | text/x-haskell |
| fmt/1130 | C3D File Format | c3d |  |
| linguist/677095381 | Cabal Config | .cabal | text/x-haskell |
| apache-httpd/16826517866150772418 | cache manifest | appcache | text/cache-manifest |
| linguist/615465151 | Caddyfile | .caddyfile |  |
| linguist/270184138 | Cadence | .cdc |  |
| apache-httpd/8411735535461836205 | caf | caf | audio/x-caf |
| linguist/891399890 | Cairo Zero | .cairo |  |
| linguist/620599567 | Cairo | .cairo |  |
| fmt/1214 | Cakewalk WRK Project | wrk |  |
| fmt/1712 | Calc602 Macro File | mc6 |  |
| fmt/1713 | Calc602 Project File | pc6 |  |
| fmt/1767 | Calc602 Project File | pc6 |  |
| fmt/1775 | Calc602 Project File | pc6 |  |
| fmt/1697 | Calc602 Spreadsheet file | bak, tc6 |  |
| fmt/1698 | Calc602 Spreadsheet file | bak, tc6 |  |
| fmt/1773 | Calc602 Spreadsheet File | bak, tc6 |  |
| fmt/1295 | Calendar Creator Event | ce3 |  |
| fmt/1296 | Calendar Creator File | cc3 |  |
| fmt/1297 | Calendar Creator File | cc5 |  |
| fmt/1298 | Calendar Creator File | bcc |  |
| x-fmt/141 | Calendar Creator Plus Data File | cce |  |
| fmt/913 | Caligari trueSpace File Format | cob, scn |  |
| fmt/914 | Caligari trueSpace File Format | cob, scn |  |
| x-fmt/28 | CALS Compressed Bitmap | cal |  |
| linguist/829207807 | CameLIGO | .mligo | text/x-ocaml |
| fmt/1852 | Camtasia Recording File | camrec |  |
| fmt/1853 | Camtasia Studio Project | camproj |  |
| fmt/1750 | Canon CIF File | cif |  |
| fmt/1749 | Canon MIF File | mif |  |
| fmt/1595 | Canon Raw | cr3 |  |
| fmt/592 | Canon RAW | cr2 |  |
| fmt/593 | Canon RAW | crw |  |
| fmt/1751 | Canon SIF File | sif |  |
| linguist/390788699 | CAP CDS | .cds |  |
| linguist/52 | Cap'n Proto | .capnp |  |
| fmt/1857 | Capture One Session File | cos |  |
| fmt/1725 | Capture One Settings File | cos |  |
| linguist/55627273 | Carbon | .carbon | text/x-go |
| fmt/1254 | Cardfile | crd |  |
| fmt/727 | Cartesian Perceptual Compression image format | cpi, cpc |  |
| linguist/53 | CartoCSS | .mss |  |
| x-fmt/224 | Cascading Style Sheet | css | text/css |
| fmt/1772 | Casio QV CAM | cam |  |
| fmt/1615 | CATIA Drawing | catdrawing |  |
| x-fmt/438 | CATIA Material Description | catmaterial |  |
| x-fmt/439 | CATIA Model (Part Description) | catpart |  |
| fmt/1714 | CATIA Model File | model |  |
| x-fmt/436 | CATIA Model | mod, model |  |
| x-fmt/440 | CATIA Product Description | catproduct |  |
| x-fmt/437 | CATIA Project | project |  |
| apache-httpd/6866278513552239343 | cbr | cbr, cba, cbt, cbz, cb7 | application/x-cbr |
| x-fmt/201 | CCITT G.711 Audio | ulaw |  |
| apache-httpd/7834014297002784124 | ccxml xml | ccxml | application/ccxml+xml |
| x-fmt/222 | CD Audio | cda | application/x-cdf |
| fmt/819 | CD-ROM/XA (eXtended Architecture) | dat |  |
| apache-httpd/3212196054821300571 | cdlink | vcd | application/x-cdlink |
| apache-httpd/5838795347239654995 | cdmi capability | cdmia | application/cdmi-capability |
| apache-httpd/5776032609182544687 | cdmi container | cdmic | application/cdmi-container |
| apache-httpd/6048803440915610366 | cdmi domain | cdmid | application/cdmi-domain |
| apache-httpd/2596188386429928103 | cdmi object | cdmio | application/cdmi-object |
| apache-httpd/7140578712814967016 | cdmi queue | cdmiq | application/cdmi-queue |
| fmt/1655 | cdrLabel Label File | clb |  |
| fmt/869 | CDX Internet Archive Index | cdx |  |
| linguist/54 | Ceylon | .ceylon |  |
| apache-httpd/11202271241335613371 | cfs compressed | cfs | application/x-cfs-compressed |
| linguist/55 | Chapel | .chpl |  |
| linguist/56 | Charity | .ch |  |
| fmt/665 | Chasys Draw image file | cd5 |  |
| fmt/1798 | CHAT Transcription Format | cha | text/x-chat |
| apache-httpd/3262656446900862880 | chat | chat | application/x-chat |
| linguist/372063053 | Checksums | .crc32, .md2, .md4, .md5, .sha1, .sha2, .sha224, .sha256, .sha256sum, .sha3, .sha384, .sha512 |  |
| apache-httpd/13641015261737520885 | chemdraw xml | cdxml | application/vnd.chemdraw+xml |
| fmt/378 | Chemical Draw Exchange Format | cdx | chemical/x-cdx |
| fmt/333 | Chemical Markup Language | cml |  |
| apache-httpd/14847441681126817338 | chess pgn | pgn | application/x-chess-pgn |
| apache-httpd/14672958157522374745 | chipnuts karaoke mmd | mmd | application/vnd.chipnuts.karaoke-mmd |
| fmt/300 | ChiWriter Document | chi |  |
| x-fmt/309 | ChiWriter Document | chi |  |
| linguist/57 | ChucK | .ck | text/x-java |
| apache-httpd/1407925950950459349 | cif | cif | chemical/x-cif |
| linguist/29176339 | CIL | .cil |  |
| apache-httpd/10930648055820933480 | cinderella | cdy | application/vnd.cinderella |
| fmt/1277 | Cindex Document | cdx, tpl |  |
| fmt/1278 | Cindex Document | ucdx, utpl |  |
| fmt/1279 | Cindex Document | ucdx, utpl |  |
| fmt/1180 | Cinema 4D | c4d |  |
| fmt/415 | Cinema 4D | c4d |  |
| fmt/540 | Cinema 4D | c4d |  |
| fmt/1716 | Cintel Raw Image/DaVinci Resolve Image | cri, dvcc |  |
| linguist/1042332086 | Circom | .circom |  |
| linguist/58 | Cirru | .cirru |  |
| linguist/59 | Clarion | .clw |  |
| fmt/741 | ClarisWorks Database | cwk |  |
| fmt/848 | ClarisWorks Database | cwk |  |
| fmt/738 | ClarisWorks Drawing | cwk |  |
| fmt/845 | ClarisWorks Drawing | cwk |  |
| fmt/742 | ClarisWorks Painting | cwk |  |
| fmt/849 | ClarisWorks Painting | cwk |  |
| fmt/740 | ClarisWorks Spreadsheet | cwk |  |
| fmt/847 | ClarisWorks Spreadsheet | cwk |  |
| fmt/739 | ClarisWorks Word Processor | cwk |  |
| fmt/846 | ClarisWorks Word Processor | cwk |  |
| fmt/746 | ClarisWorks/AppleWorks Database | cwk |  |
| fmt/743 | ClarisWorks/AppleWorks Drawing | cwk |  |
| fmt/747 | ClarisWorks/AppleWorks Painting | cwk |  |
| fmt/745 | ClarisWorks/AppleWorks Spreadsheet | cwk |  |
| fmt/744 | ClarisWorks/AppleWorks Word Processor | cwk |  |
| fmt/736 | ClarisWorks | cwk |  |
| fmt/737 | ClarisWorks | cwk |  |
| linguist/91493841 | Clarity | .clar |  |
| linguist/8 | Classic ASP | .asp |  |
| apache-httpd/6703499962162694527 | claymore | cla | application/vnd.claymore |
| linguist/60 | Clean | .icl, .dcl |  |
| linguist/61 | Click | .click |  |
| linguist/46 | CLIPS | .clp |  |
| apache-httpd/4161863740758171848 | cloanto rp9 | rp9 | application/vnd.cloanto.rp9 |
| linguist/62 | Clojure | .clj, .bb, .boot, .cl2, .cljc, .cljs, .cljs.hl, .cljscm, .cljx, .hic | text/x-clojure |
| fmt/1760 | CloneCD Control File | ccd |  |
| apache-httpd/13373727545469518801 | clonk c4group | c4g, c4d, c4f, c4p, c4u | application/vnd.clonk.c4group |
| linguist/357046146 | Closure Templates | .soy | text/x-soy |
| linguist/407996372 | Cloud Firestore Security Rules |  | text/css |
| fmt/1885 | CloudCompare Entity File | bin |  |
| apache-httpd/5331962339813467974 | cluetrust cartomobile config pkg | c11amz | application/vnd.cluetrust.cartomobile-config-pkg |
| apache-httpd/2672622462468315941 | cluetrust cartomobile config | c11amc | application/vnd.cluetrust.cartomobile-config |
| linguist/47 | CMake | .cmake, .cmake.in | text/x-cmake |
| apache-httpd/4276800422078512688 | cmdf | cmdf | chemical/x-cmdf |
| apache-httpd/15268548954167723934 | cml | cml | chemical/x-cml |
| apache-httpd/12815764328499810916 | cmu raster | ras | image/x-cmu-raster |
| apache-httpd/3283330744365083168 | cmx | cmx | image/x-cmx |
| linguist/48 | COBOL | .cob, .cbl, .ccp, .cobol, .cpy | text/x-cobol |
| linguist/321684729 | CODEOWNERS |  |  |
| linguist/424259634 | CodeQL | .ql, .qll |  |
| linguist/63 | CoffeeScript | .coffee, ._coffee, .cake, .cjsx, .iced | text/x-coffeescript |
| fmt/1587 | COKE Format (Atari Falcon) | tg1 |  |
| linguist/65 | ColdFusion CFC | .cfc |  |
| fmt/1566 | ColdFusion Markup Language | cfm |  |
| linguist/64 | ColdFusion | .cfm, .cfml |  |
| fmt/1209 | COLLADA Digital Asset Exchange (DAE) | dae | model/vnd.collada+xml |
| linguist/49 | COLLADA | .dae | text/xml |
| apache-httpd/15816773989381630238 | collection | ttc | font/collection |
| fmt/1462 | Comic Book Archive | cb7, cba, cbr, cbt, cbz |  |
| x-fmt/18 | Comma Separated Values | csv | text/csv |
| fmt/1948 | Common Data Format dotCDF | cdf |  |
| fmt/1949 | Common Data Format dotCDF | cdf |  |
| fmt/1950 | Common Data Format dotCDF | cdf |  |
| fmt/1887 | Common Instrument File (CIF) | ci1 |  |
| fmt/1888 | Common Instrument File (CIF) | ci2 |  |
| fmt/1871 | Common Interface File | cif, mca |  |
| linguist/66 | Common Lisp | .lisp, .asd, .cl, .l, .lsp, .ny, .podsl, .sexp | text/x-common-lisp |
| fmt/1944 | Common Loudspeaker Format (CLF) | cf1 |  |
| fmt/1945 | Common Loudspeaker Format (CLF) | cf2 |  |
| linguist/988547172 | Common Workflow Language | .cwl | text/x-yaml |
| apache-httpd/13251929814084281596 | commonspace | csp | application/vnd.commonspace |
| linguist/67 | Component Pascal | .cp, .cps | text/x-pascal |
| fmt/892 | Compound WordPerfect for Windows Document | wpd, doc, wp6, wp, w60 | application/vnd.wordperfect |
| fmt/2005 | Compressed MusicXML | mxl | application/vnd.recordare.musicxml |
| fmt/897 | Compressed MusicXML | mxl | application/vnd.recordare.musicxml |
| fmt/1538 | CompuServe RLE | rle |  |
| fmt/1144 | CompuServe WinCIM Message Format | plx, msg |  |
| fmt/303 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=1 |
| fmt/304 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=2 |
| fmt/305 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=3 |
| fmt/306 | Computer Graphics Metafile (Binary) | cgm | image/cgm; version=4 |
| fmt/301 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| fmt/302 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| x-fmt/142 | Computer Graphics Metafile ASCII | cgm | image/cgm |
| apache-httpd/4068569377388053849 | conference | nsc | application/x-conference |
| linguist/421026389 | CoNLL-U | .conllu, .conll |  |
| apache-httpd/17784298339216295779 | contact cmsg | cdbcmsg | application/vnd.contact.cmsg |
| fmt/1498 | Cool Edit/Adobe Audition Session File | ses |  |
| apache-httpd/14081221343819745019 | cooltalk | ice | x-conference/x-cooltalk |
| linguist/68 | Cool | .cl |  |
| linguist/69 | Coq | .coq, .v |  |
| fmt/1413 | Corel Gallery Clipart | bmf |  |
| fmt/1422 | Corel Photo House Image | cps |  |
| x-fmt/144 | Corel Photo-Paint Image | cpt |  |
| x-fmt/34 | Corel Presentation Exchange File | cmx |  |
| x-fmt/35 | Corel Presentation Exchange File | cmx |  |
| fmt/877 | Corel Presentation | shw |  |
| fmt/878 | Corel Presentation | shw |  |
| fmt/1417 | Corel Print House Document | cph, cpd |  |
| fmt/1418 | Corel Print House Document | cph, cpd |  |
| fmt/1419 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| fmt/1420 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| fmt/1421 | Corel Print House/Print Office Document | cph, cpd, cpo |  |
| fmt/431 | Corel R.A.V.E. | clk |  |
| fmt/432 | Corel R.A.V.E. | clk |  |
| x-fmt/33 | Corel R.A.V.E. | clk |  |
| x-fmt/202 | Corel Wavelet Compressed Bitmap | wi, wvl |  |
| fmt/1312 | CorelCHART Document | cch |  |
| fmt/1313 | CorelCHART Document | cch |  |
| x-fmt/310 | CorelCHART Document | cch |  |
| x-fmt/31 | CorelDraw Compressed Drawing | cdx, cjw |  |
| x-fmt/36 | CorelDraw Compressed Drawing | cpx |  |
| fmt/1924 | CorelDraw Drawing | cdr |  |
| fmt/1925 | CorelDraw Drawing | cdr |  |
| fmt/1926 | CorelDraw Drawing | cdr |  |
| fmt/1927 | CorelDraw Drawing | cdr |  |
| fmt/1928 | CorelDraw Drawing | cdr |  |
| fmt/1929 | CorelDraw Drawing | cdr |  |
| fmt/1930 | CorelDraw Drawing | cdr |  |
| fmt/1931 | CorelDraw Drawing | cdr |  |
| fmt/1932 | CorelDraw Drawing | cdr |  |
| fmt/1933 | CorelDraw Drawing | cdr |  |
| fmt/1934 | CorelDraw Drawing | cdr |  |
| fmt/427 | CorelDraw Drawing | cdr |  |
| fmt/428 | CorelDraw Drawing | cdr |  |
| fmt/429 | CorelDraw Drawing | cdr |  |
| fmt/430 | CorelDraw Drawing | cdr |  |
| fmt/464 | CorelDraw Drawing | cdr |  |
| fmt/465 | CorelDraw Drawing | cdr |  |
| fmt/466 | CorelDraw Drawing | cdr |  |
| fmt/467 | CorelDraw Drawing | cdr |  |
| x-fmt/29 | CorelDraw Drawing | cdr |  |
| x-fmt/291 | CorelDraw Drawing | cdr |  |
| x-fmt/292 | CorelDraw Drawing | cdr |  |
| x-fmt/374 | CorelDraw Drawing | cdr |  |
| x-fmt/375 | CorelDraw Drawing | cdr |  |
| x-fmt/378 | CorelDraw Drawing | cdr |  |
| x-fmt/379 | CorelDraw Drawing | cdr |  |
| x-fmt/76 | CorelDraw Pattern | pat |  |
| x-fmt/30 | CorelDraw Template | cdt |  |
| apache-httpd/13844007418067567392 | cosmocaller | cmc | application/vnd.cosmocaller |
| fmt/1676 | Covox ADPCM Audio Files | v8, cvx, v2s, v3s, v4s, vmf |  |
| apache-httpd/12237965542013894657 | cpio | cpio | application/x-cpio |
| fmt/635 | CPIO | cpio |  |
| linguist/70 | Cpp-ObjDump | .cppobjdump, .c++-objdump, .c++objdump, .cpp-objdump, .cxx-objdump |  |
| fmt/909 | CRAM File Format | cram |  |
| fmt/910 | CRAM File Format | cram |  |
| fmt/911 | CRAM File Format | cram |  |
| fmt/1736 | Creative Voice File | voc |  |
| linguist/71 | Creole | .creole |  |
| apache-httpd/17280861777505608854 | crick clicker keyboard | clkk | application/vnd.crick.clicker.keyboard |
| apache-httpd/2028789183123440400 | crick clicker palette | clkp | application/vnd.crick.clicker.palette |
| apache-httpd/13800072163877662241 | crick clicker template | clkt | application/vnd.crick.clicker.template |
| apache-httpd/18057261898033090654 | crick clicker wordbank | clkw | application/vnd.crick.clicker.wordbank |
| apache-httpd/16268303164982887344 | crick clicker | clkx | application/vnd.crick.clicker |
| apache-httpd/4179771948179943531 | criticaltools wbs xml | wbs | application/vnd.criticaltools.wbs+xml |
| linguist/705203557 | crontab |  |  |
| fmt/822 | CRT C64 Cartridge Image Format | crt |  |
| fmt/1648 | Crystal Reports File | rpt | application/x-rpt |
| fmt/334 | Crystallographic Information Framework | cif |  |
| linguist/72 | Crystal | .cr | text/x-crystal |
| apache-httpd/4465590080004403544 | csh | csh | application/x-csh |
| apache-httpd/7238172098370228130 | csml | csml | chemical/x-csml |
| linguist/424 | CSON | .cson | text/x-coffeescript |
| linguist/74 | Csound Document | .csd |  |
| linguist/75 | Csound Score | .sco |  |
| linguist/73 | Csound | .orc, .udo |  |
| linguist/50 | CSS | .css | text/css |
| fmt/800 | CSV Schema | csvs | text/csv-schema |
| linguist/51 | CSV | .csv |  |
| apache-httpd/16761428806224039353 | ctc posml | pml | application/vnd.ctc-posml |
| apache-httpd/12846344509288362340 | cu seeme | cu | application/cu-seeme |
| linguist/77 | Cuda | .cu, .cuh | text/x-c++src |
| fmt/1069 | Cue Sheet | cue |  |
| linguist/942714150 | Cue Sheet | .cue |  |
| linguist/356063509 | CUE | .cue |  |
| apache-httpd/13469738755405299163 | cups ppd | ppd | application/vnd.cups-ppd |
| apache-httpd/10354148540362929487 | curl car | car | application/vnd.curl.car |
| linguist/992375436 | cURL Config |  |  |
| apache-httpd/6263140658925115359 | curl dcurl | dcurl | text/vnd.curl.dcurl |
| apache-httpd/77337554741314457 | curl mcurl | mcurl | text/vnd.curl.mcurl |
| apache-httpd/13037335205551783511 | curl pcurl | pcurl | application/vnd.curl.pcurl |
| apache-httpd/1571811637357680016 | curl scurl | scurl | text/vnd.curl.scurl |
| apache-httpd/6202298358182500671 | curl | curl | text/vnd.curl |
| linguist/439829048 | Curry | .curry |  |
| linguist/657332628 | CWeb | .w |  |
| fmt/1557 | Cyber Paint Sequence | seq |  |
| linguist/78 | Cycript | .cy | text/javascript |
| linguist/476447814 | Cylc | .cylc |  |
| fmt/658 | Cypher Query Language | cql |  |
| linguist/850806976 | Cypher | .cyp, .cypher |  |
| linguist/79 | Cython | .pyx, .pxd, .pxi | text/x-cython |
| apache-httpd/15264676786415624429 | c | c, cc, cxx, cpp, h, hh, dic | text/x-c |
| linguist/41 | C | .c, .cats, .h, .idc | text/x-csrc |
| linguist/81 | D-ObjDump | .d-objdump |  |
| linguist/37531557 | D2 | .d2 |  |
| linguist/969323346 | Dafny | .dfy |  |
| fmt/1546 | Daisy-Dot Font File | nlq |  |
| fmt/1547 | Daisy-Dot Font File | nlq |  |
| fmt/694 | Dalvik Executable Format | dex |  |
| linguist/86 | Darcs Patch | .darcspatch, .dpatch |  |
| apache-httpd/5541150871595351962 | dart | dart | application/vnd.dart |
| linguist/87 | Dart | .dart | application/dart |
| fmt/1730 | Data File | dat |  |
| x-fmt/41 | Data Interchange Format | dif |  |
| apache-httpd/18329094975064823008 | data vision rdz | rdz | application/vnd.data-vision.rdz |
| x-fmt/197 | DataFlex Query Tag Name | tag |  |
| linguist/974514097 | DataWeave | .dwl |  |
| fmt/1851 | DAV Video Format | dav |  |
| apache-httpd/10465296028727937591 | davmount xml | davmount | application/davmount+xml |
| x-fmt/10 | dBASE Database | dbf | application/dbase |
| x-fmt/271 | dBASE Database | dbf |  |
| x-fmt/272 | dBASE Database | dbf |  |
| x-fmt/8 | dBASE Database | dbf |  |
| x-fmt/9 | dBASE Database | dbf |  |
| x-fmt/380 | dBASE for Windows database | dbf |  |
| fmt/1860 | dBASE Report Form Definition File | frm |  |
| x-fmt/311 | dBASE Text Memo | dbt |  |
| fmt/1728 | dBASE Windows Form File | wfm |  |
| linguist/527438264 | Debian Package Control File | .dsc |  |
| apache-httpd/1860797218639255065 | debian package | deb, udeb | application/x-debian-package |
| fmt/1365 | Debug File | dbg |  |
| x-fmt/286 | DEC Data Exchange File | dx | application/dec-dx. |
| x-fmt/287 | DEC WPS Plus Document | wpl |  |
| apache-httpd/9632120318872693343 | dece audio | uva, uvva | audio/vnd.dece.audio |
| apache-httpd/16059951786718315147 | dece data | uvf, uvvf, uvd, uvvd | application/vnd.dece.data |
| apache-httpd/1112684080820345302 | dece graphic | uvi, uvvi, uvg, uvvg | image/vnd.dece.graphic |
| apache-httpd/2026916056792780507 | dece hd | uvh, uvvh | video/vnd.dece.hd |
| apache-httpd/7744082576199857694 | dece mobile | uvm, uvvm | video/vnd.dece.mobile |
| apache-httpd/4607076044858037958 | dece pd | uvp, uvvp | video/vnd.dece.pd |
| apache-httpd/4112863029613538263 | dece sd | uvs, uvvs | video/vnd.dece.sd |
| apache-httpd/619002040759349425 | dece ttml xml | uvt, uvvt | application/vnd.dece.ttml+xml |
| apache-httpd/5230894226506666425 | dece unspecified | uvx, uvvx | application/vnd.dece.unspecified |
| apache-httpd/9808524822072493886 | dece video | uvv, uvvv | video/vnd.dece.video |
| apache-httpd/12792181662701773000 | dece zip | uvz, uvvz | application/vnd.dece.zip |
| x-fmt/424 | Deluxe Paint bitmap | lbm |  |
| fmt/1363 | DeluxePaint Animation File | anm |  |
| linguist/435000929 | DenizenScript | .dsc | text/x-yaml |
| apache-httpd/14103138331119806028 | denovo fcselayout link | fe_launch | application/vnd.denovo.fcselayout-link |
| x-fmt/312 | DesignCAD Drawing | dc2, dc |  |
| x-fmt/313 | DesignCAD for Windows Drawing | dw2 |  |
| x-fmt/149 | Desktop Color Separation File | dcs |  |
| linguist/412 | desktop | .desktop, .desktop.in, .service |  |
| fmt/1617 | Devicetree Blob (DTB) | dtb |  |
| apache-httpd/12491493047811765531 | dgc compressed | dgc | application/x-dgc-compressed |
| linguist/793969321 | Dhall | .dhall | text/x-haskell |
| x-fmt/381 | Dia Graphics Format | dia |  |
| fmt/1120 | DIFFRACplus Raw Data File Format | raw |  |
| fmt/1121 | DIFFRACplus Raw Data File Format | raw |  |
| linguist/88 | Diff | .diff, .patch | text/x-diff |
| fmt/1231 | DIGIDOC-XML | ddoc |  |
| fmt/1232 | DIGIDOC-XML | ddoc |  |
| fmt/1233 | DIGIDOC-XML | ddoc |  |
| linguist/82 | DIGITAL Command Language | .com |  |
| fmt/574 | Digital Imaging and Communications in Medicine File Format | dcm | application/dicom |
| fmt/193 | Digital Moving Picture Exchange Bitmap | dpx |  |
| fmt/541 | Digital Moving Picture Exchange Bitmap | dpx |  |
| fmt/152 | Digital Negative Format (DNG) | dng, tif, tiff | image/dng, image/tiff |
| fmt/1841 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| fmt/1842 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| fmt/1943 | Digital Negative Format (DNG) | dng | image/dng |
| fmt/436 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| fmt/437 | Digital Negative Format (DNG) | dng | image/dng |
| fmt/438 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| fmt/730 | Digital Negative Format (DNG) | dng | image/dng, image/tiff |
| fmt/1007 | Digital Speech Standard | dss |  |
| x-fmt/314 | Digital Terrain Elevation Data | dted, dt0, dt1, dt2, avg, min, max |  |
| x-fmt/152 | Digital Video | dv | video/dv |
| fmt/1891 | Digital Voice File (DVF) | dvf |  |
| apache-httpd/2543299767682612645 | digital winds | eol | audio/vnd.digital-winds |
| linguist/691605112 | dircolors | .dircolors |  |
| fmt/1818 | Direct Stream Digital Interchange File Format | dff |  |
| fmt/1817 | Direct Stream Digital Stream File | dsf |  |
| fmt/1040 | DirectDraw Surface | dds |  |
| fmt/957 | DirectMusic Segment File Format | sgt |  |
| fmt/958 | DirectMusic Style File Format | sty |  |
| linguist/201049282 | DirectX 3D File | .x |  |
| fmt/1399 | DiskDoubler |  |  |
| fmt/1960 | Disklavier E-Seq Music | fil, esq |  |
| fmt/255 | DjVu File Format | djvu, djv | image/vnd.djvu, image/x-djvu |
| linguist/83 | DM | .dm |  |
| fmt/1554 | DNA Sequence Chromatogram File | scf |  |
| apache-httpd/3054944743288342147 | dna | dna | application/vnd.dna |
| linguist/84 | DNS Zone | .zone, .arpa |  |
| apache-httpd/6389577377495108203 | docbook xml | dbk | application/docbook+xml |
| linguist/89 | Dockerfile | .dockerfile, .containerfile | text/x-dockerfile |
| x-fmt/315 | Document Type Definition | dtd |  |
| fmt/1827 | DOCX Strict OOXML Document | docx | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| linguist/90 | Dogescript | .djs |  |
| fmt/735 | Dolby Digital AC-3 | ac3 | audio/ac3 |
| fmt/972 | Dolby MLP Lossless Audio | mlp | audio/vnd.dolby.mlp |
| apache-httpd/16819287819186782796 | dolby mlp | mlp | application/vnd.dolby.mlp |
| fmt/572 | Domino XML Database Export | dxl |  |
| fmt/571 | Domino XML Document Export | dxl |  |
| apache-httpd/6150027229732099657 | doom | wad | application/x-doom |
| fmt/960 | DOS Sound and Music Interface Advanced Module Format | amf |  |
| linguist/111148035 | Dotenv | .env |  |
| fmt/955 | Downloadable Sounds Audio | dls | audio/dls |
| apache-httpd/6100025454960806313 | dpgraph | dpg | application/vnd.dpgraph |
| x-fmt/316 | Dr Halo Bitmap | cut |  |
| fmt/1186 | Dr. Halo Image Palette | pal |  |
| fmt/1046 | Draco File Format | drc |  |
| fmt/1946 | Draw.io Diagram (XML) File | drawio, xml |  |
| fmt/54 | Drawing Interchange Binary Format | dxb |  |
| fmt/433 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/435 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/532 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/63 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/64 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/65 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/66 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/67 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/68 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/69 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/70 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/71 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/72 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/73 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/74 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/75 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/76 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/77 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/78 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/79 | Drawing Interchange File Format (ASCII) | dxf | image/vnd.dxf |
| fmt/80 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/81 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/82 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/83 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/84 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/85 | Drawing Interchange File Format (Binary) | dxf | image/vnd.dxf |
| fmt/1389 | Drawing Interchange Format (ASCII) | dxf | image/vnd.dxf |
| fmt/1390 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| fmt/1391 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| fmt/1392 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| fmt/1393 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| fmt/1394 | Drawing Interchange Format (Binary) | dxf | image/vnd.dxf |
| x-fmt/52 | Drawing Interchange Format Style Extract | dxx |  |
| apache-httpd/10646039219402533391 | dra | dra | audio/vnd.dra |
| apache-httpd/2476393985808283697 | dreamfactory | dfac | application/vnd.dreamfactory |
| fmt/335 | Dreamweaver Lock File | lck |  |
| fmt/120 | DROID File Collection File Format | xml | text/xml |
| fmt/121 | DROID Signature File Format | xml | text/xml |
| apache-httpd/7052973897628710533 | ds keypoint | kpxx | application/vnd.ds-keypoint |
| fmt/394 | DS_Store File (MAC) | ds_store |  |
| fmt/1008 | DSS Pro | ds2 |  |
| apache-httpd/13430231427750105170 | dssc der | dssc | application/dssc+der |
| apache-httpd/10674547702623898054 | dssc xml | xdssc | application/dssc+xml |
| apache-httpd/3969505105703786494 | dtbncx xml | ncx | application/x-dtbncx+xml |
| apache-httpd/12913620085324016297 | dtbook xml | dtb | application/x-dtbook+xml |
| apache-httpd/3686352884682951143 | dtbresource xml | res | application/x-dtbresource+xml |
| linguist/85 | DTrace | .d | text/x-csrc |
| fmt/973 | DTS Coherent Acoustics (DCA) Audio | dts | audio/vnd.dts |
| apache-httpd/5184721221333372627 | dts hd | dtshd | audio/vnd.dts.hd |
| custom/3 | DuckDB | duckdb | application/vnd.duckdb.file |
| linguist/754574151 | Dune |  |  |
| apache-httpd/5012686256345022475 | dvb ait | ait | application/vnd.dvb.ait |
| apache-httpd/8181634439404576904 | dvb file | dvb | video/vnd.dvb.file |
| apache-httpd/14648134999256152995 | dvb service | svc | application/vnd.dvb.service |
| apache-httpd/14440920598703145864 | dvb subtitle | sub | text/vnd.dvb.subtitle |
| apache-httpd/2327811077702530899 | dvb subtitle | sub | image/vnd.dvb.subtitle |
| x-fmt/419 | DVD data file and backup data file | ifo, bup |  |
| linguist/91 | Dylan | .dylan, .dyl, .intr, .lid | text/x-dylan |
| apache-httpd/14560088302329463820 | dynageo | geo | application/vnd.dynageo |
| fmt/1779 | Dynamic Publisher Font File | fnt |  |
| fmt/1778 | Dynamic Publisher Picture File | pct |  |
| linguist/80 | D | .d, .di | text/x-d |
| linguist/529653389 | E-mail | .eml, .mbox | application/mbox |
| linguist/97 | Eagle | .sch, .brd | text/xml |
| fmt/372 | Earth Resource Satellite Image Header Format | ers |  |
| linguist/963512632 | Earthly |  |  |
| fmt/1665 | Easy CD Creator Layout | Roxio Easy CD Creator Layout | rcl, cl5 |  |
| linguist/342840477 | Easybuild | .eb | text/x-python |
| fmt/981 | EazyDraw File Format | ezdraw |  |
| fmt/159 | EBCDIC-US | ebcdic |  |
| linguist/430 | EBNF | .ebnf | text/x-ebnf |
| fmt/1940 | EBU Subtitling Data Exchange Format | stl |  |
| linguist/98 | Ecere Projects | .epj | application/json |
| fmt/1235 | EclipseCrossword Puzzle File | ecw |  |
| fmt/1236 | EclipseCrossword Word List File | ewl |  |
| linguist/94 | ECLiPSe | .ecl |  |
| linguist/93 | ECL | .ecl, .eclxml | text/x-ecl |
| linguist/844766630 | Ecmarkup | .html | text/html |
| apache-httpd/5284432839487308324 | ecmascript | ecma | application/ecmascript |
| apache-httpd/3616775988626861360 | ecowin chart | mag | application/vnd.ecowin.chart |
| linguist/413 | eC | .ec, .eh |  |
| linguist/925235833 | EdgeQL | .edgeql, .esdl |  |
| linguist/460509620 | Edge | .edge |  |
| linguist/96139566 | EditorConfig | .editorconfig | text/x-properties |
| linguist/342840478 | Edje Data Collection | .edc | text/x-c++src |
| linguist/414 | edn | .edn | text/x-clojure |
| fmt/1604 | EggPaint (Atari Falcon) | trp |  |
| linguist/99 | Eiffel | .e | text/x-eiffel |
| fmt/1506 | EinScan RGE 3D Range File | rge |  |
| fmt/1292 | EIOffice Document | eio |  |
| linguist/95 | EJS | .ejs, .ect, .ejs.t, .jst |  |
| fmt/1543 | ELAN Annotation File | eaf | application/eaf+xml |
| fmt/1544 | ELAN Preference File | pfsx |  |
| x-fmt/137 | Electronic Arts Music | asf |  |
| fmt/1251 | Electronically Certified Document (EDOC) | edoc | application/vnd.etsi.asic-e+zip |
| linguist/100 | Elixir | .ex, .exs |  |
| linguist/101 | Elm | .elm | text/x-elm |
| linguist/452025714 | Elvish Transcript |  |  |
| linguist/570996448 | Elvish | .elv |  |
| linguist/102 | Emacs Lisp | .el, .emacs, .emacs.desktop | text/x-common-lisp |
| fmt/1382 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| fmt/1383 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| fmt/1384 | Embedded OpenType (EOT) File Format | eot | application/vnd.ms-fontobject |
| linguist/103 | EmberScript | .em, .emberscript | text/x-coffeescript |
| apache-httpd/759975750900967096 | emma xml | emma | application/emma+xml |
| fmt/122 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| fmt/123 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| fmt/124 | Encapsulated PostScript File Format | eps, epsf, ps | application/postscript |
| fmt/1884 | Encapsulated PostScript File Format | eps, epsf | application/postscript |
| fmt/803 | Encase Image File/Expert Witness Compression File | e01 | application/encase |
| fmt/1683 | EndNote Compressed Library | enlx |  |
| fmt/1685 | EndNote Compressed Library | enlx |  |
| fmt/326 | EndNote Connection File | enz | application/x-endnote-connect, application/x-endnote-connection |
| fmt/327 | EndNote Filter File | enf |  |
| fmt/328 | EndNote Import File | enw, enr | application/x-endnote-refer |
| fmt/1682 | EndNote Library | enl |  |
| fmt/1684 | EndNote Library | enl |  |
| fmt/325 | EndNote Library | enl |  |
| fmt/324 | EndNote Style File | ens | application/x-endnote-style |
| fmt/371 | Enhanced Compression Wavelet | ecw |  |
| fmt/1856 | Enhanced Image Package | eip |  |
| fmt/1971 | Enigma Binary File (Finale) | mus |  |
| fmt/1972 | Enigma Binary File (Finale) | mus |  |
| fmt/397 | Enigma Binary File (Finale) | mus |  |
| fmt/398 | Enigma Transportable File (Finale) | etf |  |
| apache-httpd/9077911870478152775 | enliven | nml | application/vnd.enliven |
| fmt/1580 | Envision Publisher File | evp |  |
| fmt/1581 | Envision Publisher Font Files | svf |  |
| fmt/1286 | Envoy Document File | evy |  |
| fmt/1287 | Envoy Document File | evy |  |
| apache-httpd/11927020025323593929 | envoy | evy | application/x-envoy |
| apache-httpd/12706596273811258171 | epson esf | esf | application/vnd.epson.esf |
| apache-httpd/14165040414098646264 | epson msf | msf | application/vnd.epson.msf |
| apache-httpd/17134547263694623527 | epson quickanime | qam | application/vnd.epson.quickanime |
| fmt/641 | Epson Raw Image Format | erf |  |
| apache-httpd/16707635017572183154 | epson salt | slt | application/vnd.epson.salt |
| apache-httpd/2712046030815474526 | epson ssf | ssf | application/vnd.epson.ssf |
| fmt/483 | ePub format | epub | application/epub+zip |
| linguist/96 | EQ | .eq | text/x-csharp |
| fmt/195 | ERDAS IMAGINE Gray-scale Bitmap Image | gis |  |
| fmt/1563 | ERDAS Imagine Large Raster Spill File | ige |  |
| linguist/104 | Erlang | .erl, .app, .app.src, .es, .escript, .hrl, .xrl, .yrl | text/x-erlang |
| fmt/1369 | Error File | err |  |
| fmt/530 | eRuby HTML document | rhtml, rhtm |  |
| fmt/1874 | Esko ArtPro File | ap |  |
| x-fmt/218 | ESRI Arc/Info Binary Grid | adf |  |
| x-fmt/226 | ESRI Arc/Info Export File | e00, x00, e01, e02, e03, e04, e05, e06, e07, e08, e09, e10, e11, e12, e13, e14, e15, e16, e17, e18, e19, e20 |  |
| fmt/332 | ESRI Arc/View Project | apr, def |  |
| x-fmt/317 | ESRI Arc/View Project | apr |  |
| fmt/277 | ESRI Arc/View Shapefile Index | shx |  |
| x-fmt/235 | ESRI Arc/View ShapeFile | shp |  |
| fmt/1614 | Esri ArcExplorer Project File | aep |  |
| fmt/1692 | ESRI ArcGIS Raw Raster Reader/ Writer | hdr |  |
| fmt/989 | ESRI ArcGlobe Document | 3dd |  |
| fmt/1591 | ESRI ArcInfo Coverage Annotation File | txt |  |
| fmt/1594 | ESRI ArcInfo DAT File (External) | dat |  |
| fmt/1600 | ESRI ArcInfo DAT File (Internal) |  |  |
| fmt/1596 | ESRI ArcInfo Grid .nit File | nit |  |
| fmt/916 | ESRI ArcMap Document | mxd, mxt |  |
| fmt/1847 | Esri ArcMap Label file | lxp |  |
| fmt/988 | ESRI ArcScene Document | sxd |  |
| fmt/1696 | ESRI Attribute Index Files | ain |  |
| fmt/1253 | ESRI Code Page File | cpg |  |
| fmt/1625 | ESRI Colour File Format | clr |  |
| fmt/990 | ESRI File Geodatabase |  |  |
| x-fmt/225 | ESRI MapInfo Data File | mid |  |
| x-fmt/231 | ESRI MapInfo Export File | mif |  |
| fmt/1771 | ESRI Persistent Auxiliary Metadata File | xml, aux.xml |  |
| fmt/1366 | ESRI Published Map Format | pmf |  |
| fmt/1729 | Esri Shapefile Geospatial Metadata File | xml |  |
| fmt/321 | ESRI Shapefile Header Index | aih |  |
| fmt/320 | ESRI Shapefile Projection (Well-Known Text) Format | prj |  |
| fmt/319 | ESRI Spatial Index File | sbn, sbx |  |
| fmt/367 | ESRI World File Format | tfw, jgw, pgw, bpw, tifw, blw, bilw, jpgw, rasterw, btw, sdw |  |
| apache-httpd/4209892087455402036 | eszigno3 xml | es3, et3 | application/vnd.eszigno3+xml |
| fmt/1969 | ETC Express/Expression Show File | shw |  |
| linguist/880693982 | Euphoria | .e, .ex |  |
| apache-httpd/3083675910552425676 | eva | eva | application/x-eva |
| fmt/60 | Excel 95 Workbook (xls) |  |  |
| x-fmt/389 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| x-fmt/396 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| x-fmt/397 | Exchangeable Image File Format (Audio) | wav | audio/x-wav |
| fmt/1507 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| fmt/645 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| x-fmt/390 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| x-fmt/391 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| x-fmt/398 | Exchangeable Image File Format (Compressed) | jpg, jpeg | image/jpeg |
| x-fmt/387 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| x-fmt/388 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| x-fmt/399 | Exchangeable Image File Format (Uncompressed) | tif, tiff | image/tiff |
| fmt/1090 | Exclude File | exclude |  |
| fmt/688 | Executable and Linkable Format | elf, o |  |
| fmt/689 | Executable and Linkable Format | elf, o |  |
| fmt/690 | Executable and Linkable Format | elf, o |  |
| fmt/691 | Executable and Linkable Format | elf, o |  |
| fmt/1609 | exFAT (Extensible File Allocation Table) Disc Image | img |  |
| apache-httpd/3404069067379740569 | exi | exi | application/exi |
| fmt/323 | Extended Module Audio File | xm | audio/xm |
| fmt/600 | eXtensible ARchive format | xar |  |
| fmt/102 | Extensible Hypertext Markup Language | html, htm | application/xhtml+xml |
| fmt/103 | Extensible Hypertext Markup Language | htm, html | application/xhtml+xml |
| fmt/101 | Extensible Markup Language | xml | application/xml, text/xml |
| fmt/1776 | Extensible Markup Language | xml | application/xml, text/xml |
| fmt/986 | Extensible Metadata Platform Format | xmp |  |
| fmt/570 | Extensible Metadata Platform Packet | xmp |  |
| fmt/714 | Extensible Music Format | xmf, mxmf |  |
| x-fmt/281 | Extensible Stylesheet Language | xsl | application/xml |
| apache-httpd/9172159127052559283 | ezpix album | ez2 | application/vnd.ezpix-album |
| apache-httpd/6305783307604018486 | ezpix package | ez3 | application/vnd.ezpix-package |
| linguist/92 | E | .e |  |
| linguist/105 | F# | .fs, .fsi, .fsx | text/x-fsharp |
| linguist/336943375 | F* | .fst, .fsti |  |
| apache-httpd/1451447924574456576 | f4v | f4v | video/x-f4v |
| linguist/108 | Factor | .factor | text/x-factor |
| fmt/1352 | FamilyTree Maker Database | ftw, fbk |  |
| fmt/1353 | FamilyTree Maker Database | ftw, fbk |  |
| linguist/109 | Fancy | .fy, .fancypack |  |
| linguist/110 | Fantom | .fan |  |
| fmt/723 | Farandole Composer Module | far |  |
| fmt/1133 | Farbfeld Image Format | ff |  |
| fmt/1397 | FARO Laser Scan File | fls |  |
| fmt/1398 | FARO WorkSpace File | fws |  |
| apache-httpd/6089710766004459381 | fastbidsheet | fbs | image/vnd.fastbidsheet |
| fmt/1087 | FAT Disk Image | img, ima, dsk |  |
| linguist/622529198 | Faust | .dsp |  |
| fmt/1009 | FBX (Filmbox) Binary |  |  |
| fmt/1010 | FBX (Filmbox) Text | fbx |  |
| apache-httpd/4459766482740976956 | fdsn mseed | mseed | application/vnd.fdsn.mseed |
| apache-httpd/8769941306517163787 | fdsn seed | seed, dataless | application/vnd.fdsn.seed |
| fmt/889 | Feather | feather |  |
| linguist/239946126 | Fennel | .fnl |  |
| linguist/686129783 | FIGlet Font | .flf |  |
| linguist/111 | Filebench WML | .f |  |
| fmt/1059 | FileMaker Pro Database | fm |  |
| fmt/1072 | FileMaker Pro Database |  |  |
| fmt/1237 | FileMaker Pro Database | fmp12 |  |
| fmt/194 | FileMaker Pro Database | fp7 |  |
| x-fmt/318 | FileMaker Pro Database | fp3, fmp, fp, fm | application/x-filemaker |
| x-fmt/319 | FileMaker Pro Database | fp5, fmp, fp, fm |  |
| linguist/112 | Filterscript | .fs |  |
| fmt/1845 | Final Draft Document | fdx |  |
| fmt/964 | Final Draft Document | fdr |  |
| fmt/1966 | Final Writer Document | fw |  |
| fmt/1974 | Finale Notation File | musx | application/vnd.makemusic.notation |
| fmt/1973 | Finale Performance Assessment | fpa |  |
| fmt/1396 | FinePrint | fp |  |
| linguist/906694254 | FIRRTL | .fir |  |
| linguist/415 | fish | .fish |  |
| x-fmt/110 | Fixed Width Values Text File |  | text/plain |
| fmt/733 | FL Studio project file (FLP) | flp |  |
| fmt/279 | FLAC (Free Lossless Audio Codec) | flac | audio/flac |
| apache-httpd/12318121939912865041 | flac | flac | audio/x-flac |
| x-fmt/383 | Flexible Image Transport System | fits | application/fits, image/fits |
| fmt/1799 | FLExText Interlinear XML Format | flextext |  |
| apache-httpd/7213489331355052709 | fli | fli | video/x-fli |
| apache-httpd/12678262227344080642 | flographit | gph | application/vnd.flographit |
| fmt/1412 | Flow Charting Graphic Flowcharting Image | gfi |  |
| fmt/1406 | Flow Charting | cht |  |
| fmt/1407 | Flow Charting | fcd |  |
| fmt/1408 | Flow Charting | gfc |  |
| fmt/1409 | Flow Charting | fc5 |  |
| fmt/1410 | Flow Charting | fcx |  |
| fmt/1411 | Flow Charting | pdq |  |
| fmt/1737 | Flow Cytometry Standard File | fcs | application/vnd.isac.fcs |
| fmt/1785 | FLR Database File | flr |  |
| linguist/206353404 | Fluent | .ftl |  |
| apache-httpd/4578033822823314924 | fluxtime clip | ftc | application/vnd.fluxtime.clip |
| linguist/106 | FLUX | .fx, .flux |  |
| apache-httpd/5721063115553544283 | fly | fly | text/vnd.fly |
| apache-httpd/13747210224170367135 | fmi flexstor | flx | text/vnd.fmi.flexstor |
| fmt/1241 | FO File | fo | application/vnd.software602.filler.form+xml |
| fmt/1163 | Folio Definition File | def |  |
| fmt/1162 | Folio Flat File | fff |  |
| fmt/1157 | Folio Infobase File | nfo |  |
| fmt/1158 | Folio Infobase File | nfo |  |
| fmt/1159 | Folio Infobase File | nfo |  |
| fmt/1160 | Folio Shadow File | sdw |  |
| fmt/1161 | Folio Shadow File | sdw |  |
| apache-httpd/17640913485785347964 | font bdf | bdf | application/x-font-bdf |
| apache-httpd/17699946521673092095 | font ghostscript | gsf | application/x-font-ghostscript |
| apache-httpd/16860774818483330147 | font linux psf | psf | application/x-font-linux-psf |
| apache-httpd/11747660559347456894 | font pcf | pcf | application/x-font-pcf |
| apache-httpd/11006743696098534664 | font snf | snf | application/x-font-snf |
| apache-httpd/6014088398110303029 | font tdpfr | pfr | application/font-tdpfr |
| apache-httpd/9787351572380181968 | font type1 | pfa, pfb, pfm, afm | application/x-font-type1 |
| x-fmt/442 | form*Z Project File | fmz | application/octet-stream |
| linguist/113 | Formatted | .for, .eam.fs |  |
| linguist/114 | Forth | .fth, .4th, .f, .for, .forth, .fr, .frt, .fs | text/x-forth |
| linguist/761352333 | Fortran Free Form | .f90, .f03, .f08, .f95 | text/x-fortran |
| apache-httpd/14275005348846661619 | fortran | f, for, f77, f90 | text/x-fortran |
| fmt/879 | Fortran | f90, f95, f03, f, for |  |
| linguist/107 | Fortran | .f, .f77, .for, .fpp | text/x-fortran |
| fmt/1846 | Fountain Markup Language File | spmd, fountain |  |
| fmt/375 | FoxPro Compound Index File | cdx |  |
| fmt/373 | FoxPro Database | dbf |  |
| x-fmt/6 | FoxPro Database | dbf |  |
| x-fmt/7 | FoxPro Database | dbf |  |
| fmt/381 | FoxPro Project | pjx |  |
| fmt/376 | FoxPro Report | frx |  |
| x-fmt/320 | Fractal Image | fif |  |
| x-fmt/55 | Frame Vector Metafile | fmv |  |
| fmt/1173 | FrameMD5 | framemd5, md5 |  |
| x-fmt/321 | Framework Database | fw, fw2 |  |
| x-fmt/322 | Framework Database | fw3 |  |
| x-fmt/323 | Framework Database | fw4 |  |
| fmt/872 | Free Lossless Image Format (FLIF) | flif | image/flif |
| fmt/1096 | FreeArc Archive Format | arc |  |
| apache-httpd/2127115815514234195 | freearc | arc | application/x-freearc |
| linguist/472896659 | FreeBASIC | .bi, .bas | text/x-vb |
| apache-httpd/12759051345378655462 | freehand | fh, fhc, fh4, fh5, fh7 | image/x-freehand |
| x-fmt/89 | Freelance File | pre |  |
| linguist/115 | FreeMarker | .ftl |  |
| linguist/116 | Frege | .fr |  |
| apache-httpd/12395607058870399443 | frogans fnc | fnc | application/vnd.frogans.fnc |
| apache-httpd/8500864032604176996 | frogans ltf | ltf | application/vnd.frogans.ltf |
| apache-httpd/17406533731142436187 | fsc weblaunch | fsc | application/vnd.fsc.weblaunch |
| apache-httpd/17531001443158399273 | fst | fst | image/vnd.fst |
| fmt/642 | Fujifilm RAW Image Format | raf |  |
| apache-httpd/10445141258930429958 | fujitsu oasys2 | oa2 | application/vnd.fujitsu.oasys2 |
| apache-httpd/10930448860814577903 | fujitsu oasys3 | oa3 | application/vnd.fujitsu.oasys3 |
| apache-httpd/14387322337827212786 | fujitsu oasysgp | fg5 | application/vnd.fujitsu.oasysgp |
| apache-httpd/16733117196381142994 | fujitsu oasysprs | bh2 | application/vnd.fujitsu.oasysprs |
| apache-httpd/12242543921585866057 | fujitsu oasys | oas | application/vnd.fujitsu.oasys |
| apache-httpd/9711736421462602487 | fujixerox ddd | ddd | application/vnd.fujixerox.ddd |
| apache-httpd/4615680608812704211 | fujixerox docuworks binder | xbd | application/vnd.fujixerox.docuworks.binder |
| apache-httpd/7397097853789512879 | fujixerox docuworks | xdw | application/vnd.fujixerox.docuworks |
| apache-httpd/12828564092157514067 | fujixerox edmics mmr | mmr | image/vnd.fujixerox.edmics-mmr |
| apache-httpd/675748516876487325 | fujixerox edmics rlc | rlc | image/vnd.fujixerox.edmics-rlc |
| fmt/1786 | Funpaint Image File | fun, fp2, vic |  |
| linguist/97358117 | Futhark | .fut |  |
| apache-httpd/5401117936824263590 | futuresplash | spl | application/x-futuresplash |
| apache-httpd/16352059154170337709 | fuzzysheet | fzs | application/vnd.fuzzysheet |
| apache-httpd/9874601596022162347 | fvt | fvt | video/vnd.fvt |
| linguist/117 | G-code | .g, .cnc, .gco, .gcode |  |
| apache-httpd/5785170104885681682 | g3fax | g3 | image/g3fax |
| fmt/821 | G64 GCR-encoded Disk Image Format | g41, g64, g71 |  |
| fmt/1787 | G9B Graphics Format Bitmap | g9b |  |
| linguist/125 | Game Maker Language | .gml | text/x-c++src |
| linguist/290345951 | GAML | .gaml |  |
| linguist/118 | GAMS | .gms |  |
| linguist/119 | GAP | .g, .gap, .gd, .gi, .tst |  |
| fmt/1651 | Garmin Flexible and Interoperable Data Transfer File | fit |  |
| fmt/1679 | Garmin track log file | gmn |  |
| fmt/1903 | Garmin Vehicle Images File | srf |  |
| fmt/1131 | Gatan Digital Micrograph File Format (DM3) | dm3 |  |
| fmt/894 | Gaussian Input Data File | gjf |  |
| apache-httpd/16906692640029232947 | gca compressed | gca | application/x-gca-compressed |
| linguist/121 | GCC Machine Description | .md | text/x-common-lisp |
| linguist/122 | GDB | .gdb, .gdbinit |  |
| apache-httpd/13969526719176485485 | gdl | gdl | model/vnd.gdl |
| linguist/123 | GDScript | .gd |  |
| linguist/459577965 | GEDCOM | .ged |  |
| x-fmt/159 | GEM Image | img |  |
| fmt/542 | GEM Metafile Format | gem |  |
| fmt/543 | GEM Metafile Format | gem |  |
| x-fmt/215 | GEM Metafile Format | gem |  |
| linguist/907065713 | Gemfile.lock |  |  |
| linguist/310828396 | Gemini | .gmi |  |
| fmt/1770 | GenBank Flat File | gb, gbk |  |
| fmt/851 | Genealogical Data Communication (GEDCOM) Format | ged |  |
| fmt/1849 | General Purpose RAW | gpr |  |
| x-fmt/425 | Generic Library File | lib |  |
| linguist/986054050 | Genero 4gl | .4gl |  |
| linguist/902995658 | Genero per | .per |  |
| linguist/792408528 | Genie | .gs |  |
| apache-httpd/1498916370976491465 | genomatix tuxedo | txd | application/vnd.genomatix.tuxedo |
| linguist/126 | Genshi | .kid | text/xml |
| linguist/127 | Gentoo Ebuild | .ebuild | text/x-sh |
| linguist/128 | Gentoo Eclass | .eclass | text/x-sh |
| apache-httpd/12975698861315840269 | geogebra slides | ggs | application/vnd.geogebra.slides |
| apache-httpd/5451843174200806045 | geogebra tool | ggt | application/vnd.geogebra.tool |
| fmt/617 | GeoGebra | ggb | application/vnd.geogebra.file |
| fmt/618 | GeoGebra | geo | application/vnd.geogebra.file |
| fmt/619 | GeoGebra | ggb | application/vnd.geogebra.file |
| fmt/620 | GeoGebra | ggb | application/vnd.geogebra.file |
| fmt/621 | GeoGebra | ggb | application/vnd.geogebra.file |
| fmt/622 | GeoGebra | ggb | application/vnd.geogebra.file |
| fmt/155 | Geographic Tagged Image File Format (GeoTIFF) | tif, tiff | image/tiff |
| fmt/1047 | Geography Markup Language | gml | application/gml+xml |
| x-fmt/227 | Geography Markup Language | gml | application/gml+xml |
| fmt/1367 | GeoJSON | geojson | application/geo+json |
| apache-httpd/17586183174821891669 | geometry explorer | gex, gre | application/vnd.geometry-explorer |
| apache-httpd/10230197699255622710 | geonext | gxt | application/vnd.geonext |
| apache-httpd/18438421708258998726 | geoplan | g2w | application/vnd.geoplan |
| fmt/1726 | Geosoft Map Description File | mdf |  |
| apache-httpd/15897574654348769304 | geospace | g3w | application/vnd.geospace |
| fmt/664 | Gerber Format | gbr | application/vnd.gerber |
| linguist/404627610 | Gerber Image | .gbr, .cmp, .gbl, .gbo, .gbp, .gbs, .gko, .gml, .gpb, .gpt, .gtl, .gto, .gtp, .gts, .ncl, .sol |  |
| linguist/129 | Gettext Catalog | .po, .pot |  |
| linguist/76 | Gherkin | .feature, .story |  |
| fmt/615 | Gimp Image File Format | xcf |  |
| linguist/956324166 | Git Attributes |  | text/x-sh |
| linguist/807968997 | Git Config | .gitconfig | text/x-properties |
| linguist/461881235 | Git Revision List |  |  |
| fmt/1316 | GL Transmission Format (Binary) | glb | model/gltf-binary |
| fmt/1314 | GL Transmission Format (Text) | gltf | application/json |
| fmt/1315 | GL Transmission Format (Text) | gltf | application/json |
| linguist/1054258749 | Gleam | .gleam |  |
| linguist/5523150 | Glimmer JS | .gjs |  |
| linguist/95110458 | Glimmer TS | .gts |  |
| linguist/124 | GLSL | .glsl, .fp, .frag, .frg, .fs, .fsh, .fshader, .geo, .geom, .glslf, .glslv, .gs, .gshader, .rchit, .rmiss, .shader, .tesc, .tese, .vert, .vrx, .vs, .vsh, .vshader |  |
| apache-httpd/11770528808789065522 | glulx | ulx | application/x-glulx |
| linguist/997665271 | Glyph Bitmap Distribution Format | .bdf |  |
| linguist/130 | Glyph | .glf | text/x-tcl |
| apache-httpd/18355248393964917169 | gmx | gmx | application/vnd.gmx |
| fmt/1844 | GNU Image Manipulation Program Palette File | gpl |  |
| fmt/1219 | Gnumeric | gnumeric | application/x-gnumeric |
| linguist/131 | Gnuplot | .gp, .gnu, .gnuplot, .p, .plot, .plt |  |
| linguist/302957008 | GN | .gn, .gni | text/x-python |
| linguist/1054391671 | Go Checksums |  |  |
| linguist/947461016 | Go Module |  |  |
| linguist/934546256 | Go Workspace |  |  |
| fmt/1834 | GoDot 4Bit Graphics Format | 4bt |  |
| linguist/738107771 | Godot Resource | .gdnlib, .gdns, .tres, .tscn |  |
| linguist/133 | Golo | .golo |  |
| fmt/1073 | Google Document Link File | gslides, gdoc, gsheet, gdraw, gmap, gsite, gform |  |
| linguist/134 | Gosu | .gs, .gst, .gsx, .vark |  |
| linguist/132 | Go | .go | text/x-go |
| fmt/1134 | GPS Exchange Format | gpx |  |
| fmt/243 | GPS Exchange Format | gpx |  |
| apache-httpd/15669081077453213107 | gpx xml | gpx | application/gpx+xml |
| linguist/135 | Grace | .grace |  |
| linguist/432600901 | Gradle Kotlin DSL | .gradle.kts |  |
| linguist/136 | Gradle | .gradle |  |
| apache-httpd/15909748915402764967 | grafeq | gqf, gqs | application/vnd.grafeq |
| linguist/137 | Grammatical Framework | .gf | text/x-haskell |
| apache-httpd/17061360333974647667 | gramps xml | gramps | application/x-gramps-xml |
| linguist/138 | Graph Modeling Language | .gml |  |
| fmt/336 | Graphic Workshop for Windows Thumbnail File | thn |  |
| fmt/3 | Graphics Interchange Format | gif | image/gif |
| fmt/4 | Graphics Interchange Format | gif | image/gif |
| fmt/1913 | Graphisoft Archicad Project | pln, pla |  |
| fmt/1955 | Graphisoft Archicad Project | pla, pln |  |
| fmt/1914 | Graphisoft BIMx Hyper-Model | bimx |  |
| fmt/575 | GraphPad Prism | pzm |  |
| fmt/576 | GraphPad Prism | pzf |  |
| linguist/139 | GraphQL | .graphql, .gql, .graphqls |  |
| linguist/140 | Graphviz (DOT) | .dot, .gv |  |
| apache-httpd/15955169309864147200 | graphviz | gv | text/vnd.graphviz |
| fmt/284 | Gridded Binary | grb, wmo |  |
| fmt/285 | Gridded Binary | grb, wmo |  |
| apache-httpd/12441779877712447627 | groove account | gac | application/vnd.groove-account |
| apache-httpd/10596767586543562184 | groove help | ghf | application/vnd.groove-help |
| apache-httpd/14392858622867149957 | groove identity message | gim | application/vnd.groove-identity-message |
| apache-httpd/3372644123074246335 | groove injector | grv | application/vnd.groove-injector |
| apache-httpd/13266002556837886784 | groove tool message | gtm | application/vnd.groove-tool-message |
| apache-httpd/15180515305868542212 | groove tool template | tpl | application/vnd.groove-tool-template |
| apache-httpd/7294698077520185127 | groove vcard | vcg | application/vnd.groove-vcard |
| linguist/143 | Groovy Server Pages | .gsp | application/x-jsp |
| linguist/142 | Groovy | .groovy, .grt, .gtpl, .gvy | text/x-groovy |
| linguist/257856279 | GSC | .gsc, .csc, .gsh | text/x-csrc |
| fmt/362 | GSSI SIR-10 RADAN data file | dzt |  |
| fmt/1877 | GST Art File | art |  |
| fmt/1878 | GST Art File | art |  |
| fmt/1415 | GST Publisher File | dtp |  |
| fmt/1416 | GST Publisher File | dtp |  |
| apache-httpd/9798185628672492787 | gtar | gtar | application/x-gtar |
| apache-httpd/18176455083221241937 | gtw | gtw | model/vnd.gtw |
| fmt/1872 | Guitar Pro File | gtp |  |
| fmt/1873 | Guitar Pro File | gp3, gp4, gp5 |  |
| fmt/1788 | Gunpaint Image File | gun |  |
| fmt/1202 | Guymager Acquisition Info File | info |  |
| fmt/1789 | GX2 Graphics File | gx2, ega |  |
| apache-httpd/4297062800028961418 | gxf | gxf | application/gxf |
| x-fmt/266 | GZIP Format | gz, z | application/gzip |
| apache-httpd/12074909978442782798 | h261 | h261 | video/h261 |
| apache-httpd/13685448499458587184 | h263 | h263 | video/h263 |
| apache-httpd/5268943525352200654 | h264 | h264 | video/h264 |
| linguist/153 | Hack | .hack, .hh, .hhi, .php | application/x-httpd-php |
| fmt/1791 | Haiku Vector Icon Format | hvif |  |
| apache-httpd/15717357483233793640 | hal xml | hal | application/vnd.hal+xml |
| linguist/154 | Haml | .haml, .haml.deface | text/x-haml |
| apache-httpd/3355380472191294597 | handheld entertainment xml | zmm | application/vnd.handheld-entertainment+xml |
| linguist/155 | Handlebars | .handlebars, .hbs |  |
| fmt/1083 | Hangul Word Processor Document | hwp |  |
| fmt/1084 | Hangul Word Processor Document | hwp |  |
| linguist/366607477 | HAProxy | .cfg |  |
| linguist/156 | Harbour | .hb |  |
| linguist/463518941 | Hare | .ha |  |
| fmt/426 | Harris Matrix | hm |  |
| x-fmt/32 | Harvard Graphics Chart | ch3 |  |
| fmt/1491 | Harvard Graphics Presentation | prs |  |
| fmt/1492 | Harvard Graphics Presentation | pr4 |  |
| x-fmt/101 | Harvard Graphics Show | sh3 |  |
| x-fmt/324 | Harvard Graphics Show | shw |  |
| x-fmt/325 | Harvard Graphics Vector Graphics | cht |  |
| linguist/157 | Haskell | .hs, .hs-boot, .hsc | text/x-haskell |
| fmt/1062 | Hasselblad 3FR Raw Image | 3fr |  |
| linguist/158 | Haxe | .hx, .hxsl | text/x-haxe |
| apache-httpd/921806164785967428 | hbci | hbci | application/vnd.hbci |
| linguist/144 | HCL | .hcl, .nomad, .tf, .tfvars, .workflow | text/x-ruby |
| fmt/286 | HDF5 | hdf, h5, hdf5, nc |  |
| fmt/287 | HDF5 | hdf5, h5, hdf, nc |  |
| fmt/807 | HDF5 | h5, hdf, hdf5, nc |  |
| apache-httpd/16035530569936599317 | hdf | hdf | application/x-hdf |
| fmt/1041 | HDF | hdf, h4 |  |
| fmt/1790 | Help Librarian File | hlp, dat, dta |  |
| x-fmt/326 | Hewlett Packard AdvanceWrite Text File | aw |  |
| fmt/1174 | Hewlett Packard Graphics Language | 000 | application/vnd.hp-HPGL |
| x-fmt/293 | Hewlett Packard Graphics Language | hpgl | application/vnd.hp-HPGL |
| x-fmt/83 | Hewlett Packard Vector Graphic Plotter File | plt |  |
| apache-httpd/5614076178321165217 | hhe lesson player | les | application/vnd.hhe.lesson-player |
| fmt/1742 | Hierarchical File System Plus | img, dmg, toast |  |
| fmt/1105 | Hierarchical File System | img |  |
| fmt/1101 | High Efficiency Image File Format | heic | image/heif |
| linguist/931814087 | HiveQL | .q, .hql |  |
| linguist/145 | HLSL | .hlsl, .cginc, .fx, .fxh, .hlsli |  |
| fmt/1876 | HMM Packfile | pak |  |
| linguist/679725279 | HOCON | .hocon |  |
| linguist/928121743 | HolyC | .hc | text/x-csrc |
| linguist/560883276 | hoon | .hoon |  |
| linguist/231021894 | Hosts File |  |  |
| apache-httpd/15216320534825696251 | hp hpgl | hpgl | application/vnd.hp-hpgl |
| apache-httpd/2484029793804409931 | hp hpid | hpid | application/vnd.hp-hpid |
| apache-httpd/10350728140267115011 | hp hps | hps | application/vnd.hp-hps |
| apache-httpd/9662991493855678691 | hp jlyt | jlt | application/vnd.hp-jlyt |
| apache-httpd/13986790209176469882 | hp pclxl | pclxl | application/vnd.hp-pclxl |
| apache-httpd/7002675830999037402 | hp pcl | pcl | application/vnd.hp-pcl |
| fmt/1332 | HP Photo Album | albm |  |
| fmt/1212 | HP System Software Manager CVA File | cva |  |
| fmt/1423 | HP TRIM Outlook Saved Message File | vmbx, mbx |  |
| fmt/886 | HTML Components | htc |  |
| x-fmt/417 | HTML Extension File | htx |  |
| linguist/148 | HTML+ECR | .ecr | text/html |
| linguist/149 | HTML+EEX | .eex, .html.heex, .html.leex | text/html |
| linguist/150 | HTML+ERB | .erb, .erb.deface, .rhtml | application/x-erb |
| linguist/151 | HTML+PHP | .phtml | application/x-httpd-php |
| linguist/479039817 | HTML+Razor | .cshtml, .razor | text/html |
| linguist/146 | HTML | .html, .hta, .htm, .html.hl, .inc, .xht, .xhtml | text/html |
| linguist/152 | HTTP | .http | message/http |
| fmt/1843 | Human Machine Interfaces HMI File | hmi |  |
| fmt/2001 | Husqvarna / Pfaff Embroidery Stitch File | vip |  |
| fmt/2003 | Husqvarna / Premier+ Embroidery Stitch File | vp4 |  |
| fmt/2002 | Husqvarna / TruE Embroidery Stitch File | vp3 |  |
| fmt/2000 | Husqvarna Embroidery Stitch File | hus |  |
| fmt/2004 | Husqvarna-Viking Designer 1 Stitch File | shv, mhv, phv |  |
| linguist/786683730 | HXML | .hxml |  |
| apache-httpd/3133176646220210526 | hydrostatix sof data | sfd-hdstx | application/vnd.hydrostatix.sof-data |
| fmt/1490 | HyperCard Stack |  |  |
| apache-httpd/2482147210414887812 | hyperstudio | stk | application/hyperstudio |
| fmt/100 | Hypertext Markup Language | htm, html | text/html |
| fmt/471 | Hypertext Markup Language | htm, html | text/html |
| fmt/96 | Hypertext Markup Language | htm, html | text/html |
| fmt/97 | Hypertext Markup Language | htm, html | text/html |
| fmt/98 | Hypertext Markup Language | htm, html | text/html |
| fmt/99 | Hypertext Markup Language | htm, html | text/html |
| linguist/160 | HyPhy | .bf |  |
| linguist/159 | Hy | .hy |  |
| fmt/893 | i2 Analysts Notebook | anb |  |
| x-fmt/148 | IBM DisplayWrite DCA Text File | dca |  |
| x-fmt/288 | IBM DisplayWrite Document |  |  |
| x-fmt/289 | IBM DisplayWrite Document |  |  |
| x-fmt/284 | IBM DisplayWrite Final Form Text File | fft |  |
| x-fmt/285 | IBM DisplayWrite Revisable Form Text File | rft |  |
| apache-httpd/15991374402993791424 | ibm minipay | mpy | application/vnd.ibm.minipay |
| apache-httpd/5227450111622812231 | ibm modcap | afp, listafp, list3820 | application/vnd.ibm.modcap |
| apache-httpd/4568607398768371283 | ibm rights management | irm | application/vnd.ibm.rights-management |
| apache-httpd/2128091057912424249 | ibm secure container | sc | application/vnd.ibm.secure-container |
| linguist/98384424 | iCalendar | .ics, .ical | text/x-properties |
| fmt/1975 | ICC Profile | icc, icm | application/vnd.iccprofile |
| fmt/1976 | ICC Profile | icc, icm | application/vnd.iccprofile |
| fmt/1977 | ICC Profile | icc, icm | application/vnd.iccprofile |
| fmt/1793 | ICDRAW Group Icon File | ib3 |  |
| fmt/1792 | ICDRAW Single Icon File | ibi |  |
| fmt/1400 | Ichitaro Document | jtd, jtt, $td | application/x-js-taro |
| x-fmt/418 | Icon file format | ico | image/vnd.microsoft.icon, image/x-icon |
| linguist/161 | IDL | .pro, .dlm | text/x-idl |
| linguist/165 | Idris | .idr, .lidr |  |
| apache-httpd/11916099667387573904 | ief | ief | image/ief |
| fmt/1288 | IESNA LM-63 Photometric Data File | ies |  |
| apache-httpd/2229183100233788401 | igloader | igl | application/vnd.igloader |
| linguist/74444240 | Ignore List | .gitignore | text/x-sh |
| linguist/162 | IGOR Pro | .ipf |  |
| fmt/577 | Image Cytometry Standard | ics |  |
| fmt/578 | Image Cytometry Standard | ics |  |
| linguist/575143428 | ImageJ Macro | .ijm |  |
| linguist/1057618448 | Imba | .imba |  |
| fmt/1997 | IMF Package Asset Map | xml |  |
| fmt/1999 | IMF Package Composition Playlist | xml |  |
| fmt/1998 | IMF Package Packing List | xml |  |
| apache-httpd/17844121757852885447 | immervision ivp | ivp | application/vnd.immervision-ivp |
| apache-httpd/15603360700258082080 | immervision ivu | ivu | application/vnd.immervision-ivu |
| fmt/982 | iMovieProj File Format | iMovieProj |  |
| fmt/1206 | Impulse 3D Data Description Object | iob |  |
| fmt/715 | Impulse Tracker Module | it |  |
| apache-httpd/7954558507145459927 | in3d 3dml | 3dml | text/vnd.in3d.3dml |
| apache-httpd/14440943984996457076 | in3d spot | spot | text/vnd.in3d.spot |
| fmt/1184 | InDesign Markup Language Package | idml | application/vnd.adobe.indesign-idml-package |
| fmt/663 | Industry Foundation Classes XML | ifcXML |  |
| fmt/659 | Industry Foundation Classes | ifc |  |
| fmt/699 | Industry Foundation Classes | ifc |  |
| fmt/700 | Industry Foundation Classes | ifc |  |
| linguist/166 | Inform 7 | .ni, .i7x |  |
| fmt/212 | Information or Setup File | inf |  |
| x-fmt/158 | Initial Graphics Exchange Specification (IGES) | iges, igs | model/iges |
| linguist/163 | INI | .ini, .cfg, .cnf, .dof, .lektorproject, .prefs, .pro, .properties, .url | text/x-properties |
| apache-httpd/7965017775226841149 | inkml xml | ink, inkml | application/inkml+xml |
| x-fmt/95 | Inkwriter/Notetaker Document | pwi |  |
| x-fmt/81 | Inkwriter/Notetaker Template | pdt |  |
| linguist/838252715 | Ink | .ink |  |
| linguist/167 | Inno Setup | .iss, .isl |  |
| x-fmt/171 | Inset Systems Bitmap | pix |  |
| apache-httpd/12251698039906497039 | insors igm | igm | application/vnd.insors.igm |
| fmt/1647 | Inspiration Software File | isf |  |
| x-fmt/180 | Instalit Script | pvd |  |
| apache-httpd/15469217397881902467 | install instructions | install | application/x-install-instructions |
| fmt/197 | InstallShield Compiled Rules File | inx |  |
| fmt/1466 | InstallShield Executable | ex_ |  |
| x-fmt/327 | IntelliDraw Vector Graphics | idw |  |
| fmt/1675 | IntelliFont Font File | type, lib |  |
| x-fmt/328 | InterBase Database | gdb |  |
| fmt/339 | Interchange File Format 8-bit Sampled Voice | iff, 8svx |  |
| fmt/338 | Interchange File Format Interleaved Bitmap | iff, lbm |  |
| x-fmt/157 | Interchange File | iff |  |
| apache-httpd/11618853048004134807 | intercon formnet | xpw, xpx | application/vnd.intercon.formnet |
| apache-httpd/14360260915108678098 | intergeo | i2g | application/vnd.intergeo |
| x-fmt/229 | Intergraph Raster Image | ing |  |
| x-fmt/329 | Interleaf Document | doc |  |
| fmt/841 | Interleaved ADX Audio Format (AIX) | aix |  |
| fmt/1012 | INTERLIS Model File | ili |  |
| fmt/1014 | INTERLIS Model File | ili |  |
| fmt/654 | INTERLIS Model File | ili |  |
| fmt/1011 | INTERLIS Transfer File | xml, xtf |  |
| fmt/1013 | INTERLIS Transfer File | itf |  |
| fmt/653 | INTERLIS Transfer File | xtf |  |
| fmt/410 | Internet Archive | arc |  |
| x-fmt/219 | Internet Archive | arc | application/x-internet-archive |
| fmt/388 | Internet Calendar and Scheduling format | ics | text/calendar |
| fmt/358 | Internet Data Query File | idq |  |
| fmt/500 | Internet Explorer for Mac cache file | waf |  |
| fmt/278 | Internet Message Format | eml | message/rfc822 |
| fmt/1680 | INTREPID Standard Information File | isi |  |
| apache-httpd/14725031458507571431 | intu qbo | qbo | application/vnd.intu.qbo |
| apache-httpd/11066612415550178722 | intu qfx | qfx | application/vnd.intu.qfx |
| linguist/169 | Ioke | .ik |  |
| linguist/168 | Io | .io |  |
| apache-httpd/799891437585884025 | ipfix | ipfix | application/ipfix |
| apache-httpd/15706926158986829733 | ipunplugged rcprofile | rcprofile | application/vnd.ipunplugged.rcprofile |
| linguist/164 | IRC log | .irclog, .weechatlog | text/mirc |
| apache-httpd/12093715921562340635 | irepository package xml | irp | application/vnd.irepository.package+xml |
| x-fmt/194 | IRIS Graphics |  |  |
| apache-httpd/1897709426287013613 | is xpr | xpr | application/vnd.is-xpr |
| linguist/171 | Isabelle ROOT |  |  |
| linguist/170 | Isabelle | .thy |  |
| fmt/1567 | ISDOC Information System Document | isdoc |  |
| fmt/1570 | ISDOC Information System Document | isdoc |  |
| fmt/1568 | ISDOCX Information System Document | isdocx |  |
| fmt/1571 | ISDOCX Information System Document | isdocx |  |
| fmt/468 | ISO 9660 Disk Image File | iso, toast, cdr, dmg, bin |  |
| apache-httpd/9945333620512882087 | iso9660 image | iso | application/x-iso9660-image |
| fmt/975 | Jamcracker Tracker Module | jam |  |
| apache-httpd/13300239476686186637 | jam | jam | application/vnd.jam |
| linguist/1028705371 | Janet | .janet | text/x-scheme |
| linguist/447261135 | JAR Manifest |  |  |
| fmt/1125 | JASCO JWS Format | jws |  |
| linguist/180 | Jasmin | .j |  |
| x-fmt/412 | Java Archive Format | jar | application/java-archive |
| x-fmt/415 | Java Class File | class |  |
| apache-httpd/2333932034322286286 | java jnlp file | jnlp | application/x-java-jnlp-file |
| x-fmt/422 | Java Language Source Code File | java |  |
| linguist/519377561 | Java Properties | .properties | text/x-properties |
| apache-httpd/18408904452883758494 | java serialized object | ser | application/java-serialized-object |
| linguist/182 | Java Server Pages | .jsp, .tag | application/x-jsp |
| x-fmt/160 | Java Servlet Page | jsp | text/html |
| apache-httpd/774788408177624151 | java source | java | text/x-java-source |
| linguist/599494012 | Java Template Engine | .jte |  |
| apache-httpd/16922485787373200833 | java vm | class | application/java-vm |
| x-fmt/423 | JavaScript file | js | application/javascript |
| linguist/914318960 | JavaScript+ERB | .js.erb | application/javascript |
| apache-httpd/9053165588289940551 | javascript | js, mjs | text/javascript |
| linguist/183 | JavaScript | .js, ._js, .bones, .cjs, .es, .es6, .frag, .gs, .jake, .javascript, .jsb, .jscad, .jsfl, .jslib, .jsm, .jspre, .jss, .jsx, .mjs, .njs, .pac, .sjs, .ssjs, .xsjs, .xsjslib | text/javascript |
| linguist/181 | Java | .java, .jav, .jsh | text/x-java |
| linguist/316620079 | JCL | .jcl |  |
| apache-httpd/14660139996855751709 | jcp javame midlet rms | rms | application/vnd.jcp.javame.midlet-rms |
| fmt/994 | Jeffs Image Format | jif |  |
| fmt/895 | JEOL NMR Spectroscopy | jdf |  |
| linguist/774635084 | Jest Snapshot | .snap | application/javascript |
| linguist/465165328 | JetBrains MPS | .mps, .mpl, .msd | text/xml |
| linguist/173 | JFlex | .flex, .jflex |  |
| linguist/147 | Jinja | .jinja, .j2, .jinja2 | text/x-django |
| linguist/406395330 | Jison Lex | .jisonlex |  |
| linguist/284531423 | Jison | .jison |  |
| apache-httpd/18064817492816535970 | jisp | jisp | application/vnd.jisp |
| linguist/998078858 | Jolie | .ol, .iol |  |
| apache-httpd/15046818054618797090 | joost joda archive | joda | application/vnd.joost.joda-archive |
| x-fmt/392 | JP2 (JPEG 2000 part 1) | jp2 | image/jp2 |
| fmt/1794 | JPEG 2000 Codestream | j2k, jpc, j2c | image/jp2 |
| fmt/590 | JPEG Extended Range | wdp, jxr | image/jxr |
| fmt/42 | JPEG File Interchange Format | jpeg, jpe, jpg, jif, jfif, jfi | image/jpeg |
| fmt/43 | JPEG File Interchange Format | jpg, jpe, jpeg, jif, jfif, jfi | image/jpeg |
| fmt/44 | JPEG File Interchange Format | jpg, jpe, jpeg, jif, jfif, jfi | image/jpeg |
| fmt/529 | JPEG Network Graphics | jng | image/x-jng |
| fmt/1484 | JPEG XL Codestream | jxl | image/jxl |
| fmt/1485 | JPEG XL | jxl | image/jxl |
| fmt/150 | JPEG-LS | jls |  |
| apache-httpd/7742985520819096109 | jpeg | jpgv | video/jpeg |
| fmt/1964 | JPH (JPEG 2000 part 15) | jph | image/jph |
| fmt/463 | JPM (JPEG 2000 part 6) | jpm | image/jpm |
| apache-httpd/7382763164379148745 | jpm | jpm, jpgm | video/jpm |
| fmt/151 | JPX (JPEG 2000 part 2) | jpx, jpf | image/jpx |
| linguist/905371884 | jq | .jq |  |
| fmt/817 | JSON Data Interchange Format | json | application/json |
| custom/4 | JSON Lines | jsonl | application/jsonl |
| linguist/423 | JSON with Comments | .jsonc, .code-snippets, .code-workspace, .sublime-build, .sublime-color-scheme, .sublime-commands, .sublime-completions, .sublime-keymap, .sublime-macro, .sublime-menu, .sublime-mousemap, .sublime-project, .sublime-settings, .sublime-theme, .sublime-workspace, .sublime_metrics, .sublime_session | text/javascript |
| fmt/880 | JSON-LD | jsonld |  |
| linguist/175 | JSON5 | .json5 | application/json |
| linguist/177 | JSONiq | .jq | application/json |
| linguist/176 | JSONLD | .jsonld | application/json |
| apache-httpd/8812856811677659676 | jsonml json | jsonml | application/jsonml+json |
| linguist/664885656 | Jsonnet | .jsonnet, .libsonnet |  |
| linguist/174 | JSON | .json, .4DForm, .4DProject, .avsc, .geojson, .gltf, .har, .ice, .JSON-tmLanguage, .json.example, .jsonl, .mcmeta, .sarif, .tact, .tfstate, .tfstate.backup, .topojson, .webapp, .webmanifest, .yy, .yyp | application/json |
| fmt/149 | JTIP (JPEG Tiled Image Pyramid) |  |  |
| linguist/220689142 | Julia REPL |  |  |
| linguist/184 | Julia | .jl | text/x-julia |
| fmt/1908 | Jupiter Tesselation (JT) File | jt |  |
| linguist/185 | Jupyter Notebook | .ipynb | application/json |
| fmt/1119 | Jupyter Python Notebook | ipynb |  |
| x-fmt/330 | JustWrite Text Document | jw, jwt |  |
| linguist/128447695 | Just | .just |  |
| linguist/172 | J | .ijs |  |
| apache-httpd/16430238005925662096 | kahootz | ktz, ktr | application/vnd.kahootz |
| linguist/818804755 | Kaitai Struct | .ksy | text/x-yaml |
| linguist/603336474 | KakouneScript | .kak |  |
| apache-httpd/1900611674645073403 | kde karbon | karbon | application/vnd.kde.karbon |
| apache-httpd/5868147151211495659 | kde kchart | chrt | application/vnd.kde.kchart |
| apache-httpd/9081880592162353802 | kde kformula | kfo | application/vnd.kde.kformula |
| apache-httpd/12077047024898395038 | kde kivio | flw | application/vnd.kde.kivio |
| apache-httpd/3250382740988290933 | kde kontour | kon | application/vnd.kde.kontour |
| apache-httpd/15363840241960418613 | kde kpresenter | kpr, kpt | application/vnd.kde.kpresenter |
| apache-httpd/10622120689245270024 | kde kspread | ksp | application/vnd.kde.kspread |
| apache-httpd/4053381207682315711 | kde kword | kwd, kwt | application/vnd.kde.kword |
| apache-httpd/5556381308704270204 | kenameaapp | htke | application/vnd.kenameaapp |
| linguist/59716426 | KerboScript | .ks |  |
| fmt/724 | Keyhole Markup Language (Container) | kmz | application/vnd.google-earth.kmz |
| fmt/244 | Keyhole Markup Language (XML) | kml | application/vnd.google-earth.kml+xml |
| fmt/970 | Khronos Texture File | ktx | image/ktx |
| linguist/187 | KiCad Layout | .kicad_pcb, .kicad_mod, .kicad_wks | text/x-common-lisp |
| linguist/140848857 | KiCad Legacy Layout | .brd |  |
| linguist/622447435 | KiCad Schematic | .kicad_sch, .sch |  |
| linguist/692635484 | Kickstart | .ks |  |
| apache-httpd/6053998860560020025 | kidspiration | kia | application/vnd.kidspiration |
| apache-httpd/9056205338064453253 | kinar | kne, knp | application/vnd.kinar |
| linguist/188 | Kit | .kit | text/html |
| fmt/1780 | Koala MicroIllustrator Graphic File | pic |  |
| apache-httpd/14857126558942962564 | koan | skp, skd, skt, skm | application/vnd.koan |
| apache-httpd/2264720326959224348 | kodak descriptor | sse | application/vnd.kodak-descriptor |
| fmt/192 | Kodak Digital Camera Raw Image File | dcr |  |
| x-fmt/56 | Kodak FlashPix Image | fpx | image/vnd.fpx |
| fmt/211 | Kodak Photo CD Image | pcd |  |
| x-fmt/165 | Kodak PhotoCD Image |  |  |
| linguist/189 | Kotlin | .kt, .ktm, .kts | text/x-kotlin |
| fmt/999 | Krita Document Format | kra | application/x-krita |
| linguist/186 | KRL | .krl |  |
| fmt/655 | KryoFlux | raw |  |
| fmt/656 | KryoFlux | raw |  |
| linguist/225697190 | Kusto | .csl, .kql |  |
| linguist/970675279 | kvlang | .kv |  |
| linguist/194 | LabVIEW | .lvproj, .lvclass, .lvlib | text/xml |
| linguist/758480799 | Lark | .lark | text/x-ebnf |
| apache-httpd/1683673292716551156 | las las xml | lasxml | application/vnd.las.las+xml |
| linguist/195 | Lasso | .lasso, .las, .lasso8, .lasso9 |  |
| fmt/280 | LaTeX (Master document) |  |  |
| fmt/281 | LaTeX (Subdocument) |  |  |
| apache-httpd/11028599860578863387 | latex | latex | application/x-latex |
| linguist/196 | Latte | .latte | text/x-smarty |
| fmt/611 | LDAP Data Interchange Format | ldif |  |
| fmt/1336 | LEADTools Lead 1Bit Compressed Image | cmp |  |
| fmt/1337 | LEADToolsCompressed Image | cmp |  |
| fmt/1063 | Leaf Mosaic Raw Image | mos |  |
| linguist/455147478 | Lean 4 | .lean |  |
| linguist/197 | Lean | .lean, .hlean |  |
| fmt/1868 | Leapfrog Geo 3D Scene Format | lfsc |  |
| fmt/1345 | Legacy Family Tree Database | fdb |  |
| fmt/1724 | LegalDocML Document | xml |  |
| fmt/1838 | Leica Project File | lgs |  |
| fmt/1643 | Lenel Network Video Recorder File | lnr |  |
| fmt/1217 | Leonardo Image Format | leo |  |
| linguist/198 | Less | .less | text/css |
| linguist/199 | Lex | .l, .lex |  |
| linguist/190 | LFE | .lfe | text/x-common-lisp |
| fmt/626 | LHA File Format | lha, lzh |  |
| x-fmt/426 | License file | lic |  |
| fmt/587 | LifeTechnologies ABIF | abif |  |
| fmt/586 | LifeTechnologies SDS | sds |  |
| fmt/1205 | LightWave 3D Object | lw |  |
| fmt/1151 | Lightwright Show File | lw1, lw |  |
| fmt/1152 | Lightwright Show File | lw2 |  |
| fmt/1153 | Lightwright Show File | lw3 |  |
| fmt/1154 | Lightwright Show File | lw4 |  |
| fmt/1155 | Lightwright Show File | lw5 |  |
| fmt/1156 | Lightwright Show File | lw6 |  |
| linguist/1040646257 | LigoLANG | .ligo | text/x-pascal |
| linguist/200 | LilyPond | .ly, .ily |  |
| linguist/201 | Limbo | .b, .m |  |
| linguist/202 | Linker Script | .ld, .lds, .x |  |
| linguist/203 | Linux Kernel Module | .mod |  |
| fmt/1672 | Linux/i386 Binary Executable File ZMAGIC | so, o |  |
| linguist/204 | Liquid | .liquid |  |
| linguist/205 | Literate Agda | .lagda |  |
| linguist/206 | Literate CoffeeScript | .litcoffee, .coffee.md |  |
| linguist/207 | Literate Haskell | .lhs | text/x-literate-haskell |
| linguist/891017 | LiveCode Script | .livecodescript |  |
| fmt/1920 | LiveCode Stack | rev, livecode |  |
| fmt/1921 | LiveCode Stack | rev, livecode |  |
| fmt/1922 | LiveCode Stack |  |  |
| fmt/1923 | LiveCode Stack | rev, livecode |  |
| linguist/208 | LiveScript | .ls, ._ls | text/x-livescript |
| apache-httpd/13042316040512874645 | llamagraphics life balance desktop | lbd | application/vnd.llamagraphics.life-balance.desktop |
| apache-httpd/5029791183561743911 | llamagraphics life balance exchange xml | lbe | application/vnd.llamagraphics.life-balance.exchange+xml |
| linguist/191 | LLVM | .ll |  |
| fmt/1310 | LocoFile |  |  |
| fmt/1304 | LocoScript Document |  |  |
| fmt/1305 | LocoScript Document |  |  |
| fmt/1306 | LocoScript Document |  |  |
| fmt/1307 | LocoScript Document |  |  |
| fmt/1308 | LocoScript PC |  |  |
| fmt/1309 | LocoScript Professional |  |  |
| fmt/389 | Log ASCII Standard Format | las |  |
| fmt/390 | Log ASCII Standard Format | las |  |
| fmt/391 | Log ASCII Standard Format | las |  |
| x-fmt/62 | Log File | log |  |
| fmt/804 | Logical File Evidence Format | l01 |  |
| linguist/209 | Logos | .xm, .x, .xi |  |
| linguist/210 | Logtalk | .lgt, .logtalk |  |
| linguist/192 | LOLCODE | .lol |  |
| linguist/211 | LookML | .lkml, .lookml | text/x-yaml |
| linguist/212 | LoomScript | .ls |  |
| apache-httpd/5927135927038744495 | lost xml | lostxml | application/lost+xml |
| x-fmt/82 | Lotus 1-2-3 Chart | pic |  |
| x-fmt/331 | Lotus 1-2-3 Spreadsheet Formatting File | fm1, fmt |  |
| x-fmt/332 | Lotus 1-2-3 Spreadsheet Formatting File | fm3 |  |
| fmt/1452 | Lotus 1-2-3 Worksheet | 123 | application/vnd.lotus-1-2-3, application/x-123 |
| fmt/1453 | Lotus 1-2-3 Worksheet | 123 | application/vnd.lotus-1-2-3, application/x-123 |
| x-fmt/114 | Lotus 1-2-3 Worksheet | wk1, wk2 | application/vnd.lotus-1-2-3, application/x-123 |
| x-fmt/115 | Lotus 1-2-3 Worksheet | wk3 | application/lotus123, application/vnd.lotus-1-2-3 |
| x-fmt/116 | Lotus 1-2-3 Worksheet | wk4 | application/lotus123, application/vnd.lotus-1-2-3 |
| x-fmt/117 | Lotus 1-2-3 Worksheet | wks | application/vnd.lotus-1-2-3, application/x-123 |
| x-fmt/212 | Lotus 1-2-3 Worksheet |  |  |
| x-fmt/333 | Lotus Approach View File | apr | application/vnd.lotus-approach |
| x-fmt/334 | Lotus Approach View File | apt | application/vnd.lotus-approach |
| fmt/1216 | Lotus Freelance Show | prz | application/vnd.lotus-freelance |
| x-fmt/335 | Lotus Freelance Smartmaster Graphics | mas | application/vnd.lotus-freelance |
| apache-httpd/5593557214741015173 | lotus freelance | pre | application/vnd.lotus-freelance |
| x-fmt/336 | Lotus Notes Database | ns2, nsf | application/vnd.lotus-notes |
| x-fmt/337 | Lotus Notes Database | ns3, nsf | application/vnd.lotus-notes |
| x-fmt/338 | Lotus Notes Database | ns4, nsf | application/vnd.lotus-notes |
| x-fmt/339 | Lotus Notes File | box |  |
| apache-httpd/12260496078258194652 | lotus organizer | org | application/vnd.lotus-organizer |
| fmt/1938 | Lotus Screencam Data File | scm | application/vnd.lotus-screencam |
| fmt/340 | Lotus WordPro Document | lwp | application/lwp, application/vnd.lotus-wordpro |
| x-fmt/340 | Lotus WordPro Document | lwp | application/lwp, application/vnd.lotus-wordpro |
| linguist/193 | LSL | .lsl, .lslp |  |
| linguist/1013566805 | LTspice Symbol | .asy | text/x-spreadsheet |
| linguist/365050359 | Luau | .luau | text/x-lua |
| linguist/213 | Lua | .lua, .fcgi, .nse, .p8, .pd_lua, .rbxs, .rockspec, .wlua | text/x-lua |
| apache-httpd/13708406258791595207 | lucent voice | lvp | audio/vnd.lucent.voice |
| apache-httpd/487741630637929822 | lzh compressed | lzh, lha | application/x-lzh-compressed |
| fmt/1055 | M2TS | mts, m2ts |  |
| linguist/216 | M4Sugar | .m4 |  |
| apache-httpd/6948131469390528991 | m4v | m4v | video/x-m4v |
| linguist/215 | M4 | .m4, .mc |  |
| apache-httpd/7253703184866025307 | mac compactpro | cpt | application/mac-compactpro |
| linguist/34167825 | Macaulay2 | .m2 |  |
| fmt/1761 | MacBinary |  |  |
| fmt/1762 | MacBinary | bin |  |
| fmt/1763 | MacBinary | bin |  |
| fmt/1819 | MacCaption File | mcc |  |
| fmt/1820 | MacCaption File | mcc |  |
| fmt/1821 | MacCaption Project | cca |  |
| fmt/1425 | MacDraw |  |  |
| fmt/1426 | MacDraw |  |  |
| fmt/1427 | MacDraw |  |  |
| fmt/1428 | MacDraw |  |  |
| fmt/692 | Mach-O |  |  |
| fmt/693 | Mach-O |  |  |
| fmt/341 | Macintosh PICT Image | pct, pict, pic | image/x-pict |
| x-fmt/80 | Macintosh PICT Image | pct, pict | image/x-pict |
| x-fmt/14 | Macintosh Text File |  | text/plain |
| x-fmt/175 | MacPaint Graphics | pnt |  |
| fmt/1429 | MacPaint Image |  |  |
| x-fmt/161 | MacPaint Image | mac |  |
| apache-httpd/5659107803445623294 | macports portpkg | portpkg | application/vnd.macports.portpkg |
| fmt/487 | Macro Enabled Microsoft Powerpoint | pptm | application/vnd.ms-powerpoint.presentation.macroEnabled.12 |
| fmt/523 | Macro enabled Microsoft Word Document OOXML | docm | application/vnd.ms-word.document.macroEnabled.12 |
| fmt/486 | Macromedia (Adobe) Director Compressed Resource file | dcr |  |
| fmt/317 | Macromedia Director | dir, dxr | application/x-director |
| x-fmt/341 | Macromedia Director | dir, dxr | application/x-director |
| fmt/104 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/105 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/106 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/107 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/108 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/109 | Macromedia Flash | swf | application/x-shockwave-flash |
| fmt/110 | Macromedia Flash | swf | application/x-shockwave-flash |
| x-fmt/382 | Macromedia FLV | flv | video/x-flv |
| fmt/400 | Macromedia FreeHand MX | fh11 |  |
| fmt/544 | Macromedia FreeHand | fh7 |  |
| fmt/545 | Macromedia FreeHand | fh8 |  |
| fmt/546 | Macromedia FreeHand | fh9 |  |
| fmt/547 | Macromedia FreeHand | fh10 |  |
| x-fmt/53 | Macromedia Freehand | fh5, fh4 |  |
| apache-httpd/13249549495329969252 | mads xml | mads | application/mads+xml |
| fmt/1464 | Maestro Music File |  |  |
| fmt/1472 | Magic Shadow Archiver Disk Image File | msa | application/vnd.msa-disk-image |
| fmt/976 | MagicaVoxel Vox format | vox |  |
| fmt/930 | Magick Image File Format | mif |  |
| linguist/220 | Makefile | .mak, .d, .make, .makefile, .mk, .mkfile | text/x-cmake |
| fmt/1469 | MAKIchan Graphics File | mki, mag, max |  |
| linguist/221 | Mako | .mako, .mao |  |
| x-fmt/221 | MapBrowser/MapWriter Vector Map Data | cbd |  |
| fmt/1256 | MapInfo Workspace File | wor |  |
| fmt/915 | Mapsforge Binary Map File Format | map |  |
| fmt/1875 | Maptech BSB Documentation File | bsb, kap |  |
| fmt/1483 | Mar Archive | mar, mac |  |
| apache-httpd/1512096571599294824 | marcxml xml | mrcx | application/marcxml+xml |
| apache-httpd/15435368578074645444 | marc | mrc | application/marc |
| fmt/1149 | Markdown | md, markdown | text/markdown |
| linguist/222 | Markdown | .md, .livemd, .markdown, .mdown, .mdwn, .mkd, .mkdn, .mkdown, .ronn, .scd, .workbook | text/x-gfm |
| linguist/932782397 | Marko | .marko | text/html |
| linguist/223 | Mask | .mask |  |
| fmt/1900 | Mass Spectrometry Markup Language | mxml |  |
| fmt/200 | Material Exchange Format | mxf | application/mxf |
| fmt/783 | Material Exchange Format | mxf | application/mxf |
| fmt/784 | Material Exchange Format | mxf | application/mxf |
| fmt/785 | Material Exchange Format | mxf | application/mxf |
| fmt/786 | Material Exchange Format | mxf | application/mxf |
| fmt/787 | Material Exchange Format | mxf | application/mxf |
| fmt/788 | Material Exchange Format | mxf | application/mxf |
| fmt/789 | Material Exchange Format | mxf | application/mxf |
| fmt/790 | Material Exchange Format | mxf | application/mxf |
| fmt/791 | Material Exchange Format | mxf | application/mxf |
| fmt/931 | Mathcad Document | mcd |  |
| fmt/932 | Mathcad Document | xmcd |  |
| fmt/201 | Mathematica Notebook | nb | application/mathematica |
| linguist/224 | Mathematica | .mathematica, .cdf, .m, .ma, .mt, .nb, .nbp, .wl, .wlt | text/x-mathematica |
| apache-httpd/3699261783011718340 | mathml xml | mathml | application/mathml+xml |
| fmt/1550 | MATLAB Mat File | mat |  |
| fmt/806 | MATLAB Mat File | mat, fig |  |
| fmt/828 | MATLAB Mat File | mat, fig |  |
| fmt/1678 | MATLAB Script File | m |  |
| linguist/225 | MATLAB | .matlab, .m | text/x-octave |
| apache-httpd/4395470199143842936 | matroska | mkv, mk3d, mks | video/x-matroska |
| apache-httpd/5069105610641688063 | matroska | mka | audio/x-matroska |
| fmt/569 | Matroska | mkv, mk3d, mka, mks |  |
| linguist/226 | Maven POM |  | text/xml |
| linguist/217 | MAXScript | .ms, .mcr |  |
| fmt/1146 | Maxwell Render Image Format | mxi |  |
| fmt/1145 | Maxwell Render Material File | mxm |  |
| fmt/1147 | Maxwell Render Scene File Format | mxs |  |
| linguist/227 | Max | .maxpat, .maxhelp, .maxproj, .mxt, .pat | application/json |
| fmt/863 | Maya ASCII File Format | ma |  |
| fmt/861 | Maya Binary File Format | mb |  |
| fmt/862 | Maya Binary File Format | mb |  |
| fmt/1168 | Maya Icons or Swatches file | icons, swatches |  |
| fmt/1169 | Maya IFF Image File | iff, ico |  |
| fmt/720 | MBOX | mbox | application/mbox |
| apache-httpd/1187713301663608044 | mcd | mcd | application/vnd.mcd |
| linguist/462488745 | mcfunction | .mcfunction |  |
| fmt/993 | MD5 File | md5 |  |
| linguist/512838272 | MDX | .mdx | text/x-gfm |
| apache-httpd/4072025194511321041 | medcalcdata | mc1 | application/vnd.medcalcdata |
| fmt/1758 | Media Descriptor File | mdf |  |
| fmt/1759 | Media Descriptor Sidecar File | mds |  |
| fmt/1765 | Media Hash List | mhl |  |
| fmt/648 | Media View Pro | mpcatalog |  |
| apache-httpd/5549479812584870908 | mediaservercontrol xml | mscml | application/mediaservercontrol+xml |
| apache-httpd/5788543607611052916 | mediastation cdkey | cdkey | application/vnd.mediastation.cdkey |
| fmt/1958 | Melco OFM Project | ofm |  |
| fmt/1959 | Melco OFM Project | ofm |  |
| fmt/1892 | Memory Stick Voice File (MSV)/Digital Voice File (DVF) | msv, dvf |  |
| fmt/1890 | Memory Stick Voice File (MSV) | msv |  |
| linguist/229 | Mercury | .m, .moo |  |
| linguist/385992043 | Mermaid | .mmd, .mermaid |  |
| apache-httpd/6674260812445752722 | mesh | msh, mesh, silo | model/mesh |
| linguist/799141244 | Meson |  |  |
| fmt/1918 | MetaCard Stack | mc, rev |  |
| apache-httpd/7305870426760626928 | metalink xml | metalink | application/metalink+xml |
| apache-httpd/2965419729449071055 | metalink4 xml | meta4 | application/metalink4+xml |
| linguist/230 | Metal | .metal | text/x-c++src |
| apache-httpd/16965387237143203176 | mets xml | mets | application/mets+xml |
| apache-httpd/12340112869033537762 | mfer | mwf | application/vnd.mfer |
| apache-httpd/10582393778901387068 | mfmp | mfm | application/vnd.mfmp |
| x-fmt/429 | MHTML | mht, mhtml | multipart/related |
| x-fmt/151 | Micrografx Designer | dsf |  |
| x-fmt/296 | Micrografx Designer | drw |  |
| x-fmt/294 | Micrografx Draw | drw |  |
| x-fmt/295 | Micrografx Draw | drw, drt |  |
| x-fmt/47 | Micrografx Draw | drw |  |
| apache-httpd/6898984041858146169 | micrografx flo | flo | application/vnd.micrografx.flo |
| fmt/1907 | Micrografx Icon File | icn |  |
| apache-httpd/15630303916909994538 | micrografx igx | igx | application/vnd.micrografx.igx |
| fmt/1481 | Micrografx In-A-Vision Drawing | pic |  |
| fmt/1805 | Microsoft Access Database File | mdb, mda |  |
| fmt/1806 | Microsoft Access Database File | mdb, mda |  |
| fmt/275 | Microsoft Access Database File | accdb |  |
| x-fmt/238 | Microsoft Access Database File | mdb, mda, mde, mdt |  |
| x-fmt/239 | Microsoft Access Database File | mdb, mda, mdt, mde |  |
| x-fmt/240 | Microsoft Access Database File | mdb, mde |  |
| x-fmt/241 | Microsoft Access Database File | mdb, mde |  |
| x-fmt/66 | Microsoft Access Database File | mdb, mda |  |
| fmt/1807 | Microsoft Access Encrypted Database File | mdb, mda |  |
| fmt/1808 | Microsoft Access Encrypted Database File | mdb, mda |  |
| fmt/1809 | Microsoft Access Encrypted Database File | mdb, mda |  |
| fmt/1258 | Microsoft Access Workgroup Information File | mdw |  |
| fmt/1893 | Microsoft Agent File | acs |  |
| fmt/386 | Microsoft Animated Cursor Format | ani |  |
| fmt/634 | Microsoft Compiled HTML Help | chm, chw | application/vnd.ms-htmlhelp |
| linguist/800983837 | Microsoft Developer Studio Project | .dsp |  |
| fmt/881 | Microsoft Document Imaging File Format | mdi | image/vnd.ms-modi |
| fmt/55 | Microsoft Excel 2.x Worksheet (xls) | xls | application/vnd.ms-excel |
| fmt/62 | Microsoft Excel 2000-2003 Workbook (xls) | xlw, xls | application/vnd.ms-excel |
| fmt/56 | Microsoft Excel 3.0 Worksheet (xls) | xls | application/vnd.ms-excel |
| fmt/58 | Microsoft Excel 4.0 Workbook (xls) | xlw | application/vnd.ms-excel |
| fmt/57 | Microsoft Excel 4.0 Worksheet (xls) | xls | application/vnd.ms-excel |
| fmt/59 | Microsoft Excel 5.0/95 Workbook (xls) | xlw, xls | application/vnd.ms-excel |
| fmt/61 | Microsoft Excel 97 Workbook (xls) | xls, xlw | application/vnd.ms-excel |
| x-fmt/124 | Microsoft Excel Add-In | xla, xll |  |
| x-fmt/23 | Microsoft Excel Backup | xlk |  |
| fmt/553 | Microsoft Excel Chart | xlc | application/vnd.ms-excel |
| fmt/554 | Microsoft Excel Chart | xlc | application/vnd.ms-excel |
| x-fmt/126 | Microsoft Excel Chart | xlc |  |
| fmt/172 | Microsoft Excel for Macintosh |  |  |
| fmt/173 | Microsoft Excel for Macintosh |  |  |
| fmt/174 | Microsoft Excel for Macintosh |  |  |
| fmt/175 | Microsoft Excel for Macintosh |  |  |
| fmt/176 | Microsoft Excel for Macintosh |  |  |
| fmt/177 | Microsoft Excel for Macintosh |  |  |
| fmt/178 | Microsoft Excel for Macintosh |  |  |
| fmt/214 | Microsoft Excel for Windows | xlsx | application/vnd.openxmlformats-officedocument.spreadsheetml.sheet |
| fmt/628 | Microsoft Excel Macro-Enabled Add-In | xlam | application/vnd.ms-excel.addin.macroEnabled.12 |
| fmt/627 | Microsoft Excel Macro-Enabled Template | xltm | application/vnd.ms-excel.template.macroEnabled.12 |
| fmt/445 | Microsoft Excel Macro-Enabled | xlsm | application/vnd.ms-excel.sheet.macroEnabled.12 |
| fmt/555 | Microsoft Excel Macro | xlm | application/vnd.ms-excel |
| fmt/556 | Microsoft Excel Macro | xlm | application/vnd.ms-excel |
| x-fmt/123 | Microsoft Excel Macro | xla, xlm | application/vnd.ms-excel |
| fmt/595 | Microsoft Excel Non-XML Binary Workbook | xlsb | application/vnd.ms-excel.sheet.binary.macroEnabled.12 |
| x-fmt/46 | Microsoft Excel ODBC Query | dqy |  |
| x-fmt/74 | Microsoft Excel OLAP Query | oqy |  |
| x-fmt/97 | Microsoft Excel OLE DB Query | rqy |  |
| fmt/598 | Microsoft Excel Template | xltx | application/vnd.openxmlformats-officedocument.spreadsheetml.template |
| x-fmt/17 | Microsoft Excel Template | xlt | application/vnd.ms-excel |
| x-fmt/125 | Microsoft Excel Toolbar | xlb |  |
| x-fmt/58 | Microsoft Excel Web Query | iqy |  |
| fmt/1858 | Microsoft Excel Workspace File | xlw | application/vnd.ms-excel |
| x-fmt/128 | Microsoft Excel Workspace File | xlw |  |
| fmt/647 | Microsoft Expression Media | ivc |  |
| x-fmt/242 | Microsoft FoxPro Database | dbf |  |
| x-fmt/172 | Microsoft FoxPro Library | plb |  |
| x-fmt/342 | Microsoft FoxPro Memo | fpt, frt, vct, pjt |  |
| fmt/359 | Microsoft Front Page Binary Tree Index | btr |  |
| fmt/288 | Microsoft Front Page Server Extension Configuration |  |  |
| fmt/218 | Microsoft FrontPage | lck |  |
| fmt/1656 | Microsoft Help Contents File | cnt |  |
| x-fmt/454 | Microsoft Internet Shortcut | url | text/plain |
| fmt/475 | Microsoft Management Console Snap-in Control file | msc |  |
| fmt/1362 | Microsoft MapPoint Document | ptm |  |
| fmt/162 | Microsoft Multiplan | mod |  |
| fmt/777 | Microsoft Network Monitor Packet Capture | cap |  |
| fmt/778 | Microsoft Network Monitor Packet Capture | cap |  |
| fmt/237 | Microsoft Office Binder File for Windows | obd |  |
| fmt/240 | Microsoft Office Binder File for Windows | obd |  |
| fmt/238 | Microsoft Office Binder Template for Windows | obt |  |
| fmt/241 | Microsoft Office Binder Template for Windows | obt |  |
| fmt/239 | Microsoft Office Binder Wizard for Windows | obz |  |
| fmt/242 | Microsoft Office Binder Wizard for Windows | obz |  |
| fmt/494 | Microsoft Office Encrypted Document | xlsx, pptx, docx |  |
| fmt/1677 | Microsoft Office File List | xml |  |
| fmt/189 | Microsoft Office Open XML |  |  |
| fmt/473 | Microsoft Office Owner File | doc, docx |  |
| fmt/524 | Microsoft Office Theme | thmx | application/vnd.ms-officetheme |
| fmt/987 | Microsoft OneNote Package File | onepkg |  |
| fmt/637 | Microsoft OneNote | one | application/msonenote |
| x-fmt/73 | Microsoft Outlook Address Book | olk |  |
| x-fmt/430 | Microsoft Outlook Email Message | msg, oft |  |
| x-fmt/75 | Microsoft Outlook Personal Address Book | pab |  |
| x-fmt/248 | Microsoft Outlook Personal Folders (ANSI) | pst |  |
| x-fmt/249 | Microsoft Outlook Personal Folders (Unicode) | pst |  |
| x-fmt/250 | Microsoft Outlook Personal Folders |  |  |
| x-fmt/251 | Microsoft Outlook Personal Folders |  |  |
| fmt/912 | Microsoft Paint | msp |  |
| x-fmt/214 | Microsoft Paint | msp |  |
| fmt/594 | Microsoft PhotoDraw | mix | image/vnd.mix |
| fmt/936 | Microsoft Picture It! Image File | mix | image/vnd.mix |
| x-fmt/86 | Microsoft Powerpoint Add-In | ppa |  |
| x-fmt/84 | Microsoft Powerpoint Design Template | pot |  |
| fmt/179 | Microsoft PowerPoint for Macintosh | ppt | application/vnd.ms-powerpoint |
| fmt/180 | Microsoft PowerPoint for Macintosh |  |  |
| fmt/181 | Microsoft PowerPoint for Macintosh | ppt | application/vnd.ms-powerpoint |
| fmt/182 | Microsoft PowerPoint for Macintosh |  |  |
| fmt/1866 | Microsoft Powerpoint for Macintosh | ppt | application/vnd.ms-PowerPoint |
| fmt/1867 | Microsoft Powerpoint for Macintosh | ppt | application/vnd.ms-PowerPoint |
| fmt/215 | Microsoft Powerpoint for Windows | pptx | application/vnd.openxmlformats-officedocument.presentationml.presentation |
| x-fmt/177 | Microsoft PowerPoint Graphics File | ppi |  |
| fmt/633 | Microsoft PowerPoint Macro-Enabled Add-In | ppam | application/vnd.ms-powerpoint.addin.macroEnabled.12 |
| fmt/630 | Microsoft PowerPoint Macro-Enabled Show | ppsm | application/vnd.ms-powerpoint.slideshow.macroEnabled.12 |
| fmt/636 | Microsoft PowerPoint Macro-Enabled Slide | sldm | application/vnd.ms-powerpoint.slide.macroEnabled.12 |
| fmt/632 | Microsoft PowerPoint Macro-Enabled Template | potm | application/vnd.ms-powerpoint.template.macroEnabled.12 |
| x-fmt/216 | Microsoft Powerpoint Packaged Presentation | ppz |  |
| x-fmt/87 | Microsoft Powerpoint Presentation Show | pps | application/vnd.ms-powerpoint |
| fmt/125 | Microsoft Powerpoint Presentation | ppt | application/vnd.ms-powerpoint |
| fmt/126 | Microsoft Powerpoint Presentation | ppt | application/vnd.ms-powerpoint |
| fmt/1747 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-PowerPoint |
| fmt/1748 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-PowerPoint |
| x-fmt/88 | Microsoft PowerPoint Presentation | ppt | application/vnd.ms-powerpoint |
| fmt/629 | Microsoft PowerPoint Show | ppsx | application/vnd.openxmlformats-officedocument.presentationml.slideshow |
| fmt/631 | Microsoft PowerPoint Template | potx | application/vnd.openxmlformats-officedocument.presentationml.template |
| x-fmt/90 | Microsoft Print File | prn |  |
| fmt/1078 | Microsoft Program Database | pdb |  |
| fmt/1079 | Microsoft Program Database | pdb |  |
| fmt/342 | Microsoft Project Export File | mpx | application/x-project |
| fmt/343 | Microsoft Project Export File | mpx | application/x-project |
| x-fmt/232 | Microsoft Project Export File | mpx | application/x-project |
| fmt/440 | Microsoft Project | mpp | application/vnd.ms-project |
| fmt/725 | Microsoft Project | mpp | application/vnd.ms-project |
| x-fmt/243 | Microsoft Project | mpp | application/vnd.ms-project |
| x-fmt/244 | Microsoft Project | mpp | application/vnd.ms-project |
| x-fmt/245 | Microsoft Project | mpp | application/vnd.ms-project |
| x-fmt/246 | Microsoft Project |  |  |
| x-fmt/247 | Microsoft Project | mpp | application/vnd.ms-project |
| fmt/1043 | Microsoft PRX File | prx |  |
| fmt/1839 | Microsoft Publisher Packaged Document | puz |  |
| fmt/1511 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/1512 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/1513 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/1514 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/1515 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/1516 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/252 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/253 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/254 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/255 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/256 | Microsoft Publisher | pub | application/x-mspublisher |
| x-fmt/257 | Microsoft Publisher | pub | application/x-mspublisher |
| fmt/867 | Microsoft Reader eBook | lit |  |
| fmt/1303 | Microsoft Shell Scrap Object File | shs |  |
| x-fmt/106 | Microsoft Symbolic Link (SYLK) File | slk |  |
| fmt/442 | Microsoft Visio (generic) | vsd | application/vnd.visio |
| fmt/1508 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| fmt/1509 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| fmt/1510 | Microsoft Visio Drawing | vsd, vst, vss, vsw | application/vnd.visio |
| fmt/443 | Microsoft Visio Drawing | vsd | application/vnd.visio |
| fmt/924 | Microsoft Visio Drawing | vsdx | application/vnd.ms-visio.drawing.main+xml |
| x-fmt/113 | Microsoft Visio Drawing | vsd, vst, vss | application/vnd.visio |
| x-fmt/258 | Microsoft Visio Drawing | vsd, vss, vst | application/vnd.visio |
| x-fmt/259 | Microsoft Visio Drawing |  |  |
| fmt/927 | Microsoft Visio Macro-Enabled Drawing | vsdm | application/vnd.ms-visio.drawing.macroEnabled.main+xml |
| fmt/928 | Microsoft Visio Macro-Enabled Stencil | vssm | application/vnd.ms-visio.stencil.macroEnabled.main+xml |
| fmt/929 | Microsoft Visio Macro-Enabled Template | vstm | application/vnd.ms-visio.template.macroEnabled.main+xml |
| fmt/925 | Microsoft Visio Stencil | vssx | application/vnd.ms-visio.stencil.main+xml |
| fmt/926 | Microsoft Visio Template | vstx | application/vnd.ms-visio.template.main+xml |
| fmt/216 | Microsoft Visio XML Drawing | vdx | application/vnd.visio |
| fmt/379 | Microsoft Visual FoxPro Class Library | vcx |  |
| fmt/384 | Microsoft Visual FoxPro database container (memo files) | dct |  |
| fmt/382 | Microsoft Visual FoxPro database container (table files) | dbc |  |
| fmt/374 | Microsoft Visual FoxPro Database Table File | dbf |  |
| fmt/380 | Microsoft Visual FoxPro Project | pjx |  |
| fmt/377 | Microsoft Visual FoxPro Report | frx |  |
| x-fmt/343 | Microsoft Visual FoxPro Table | dbx |  |
| x-fmt/179 | Microsoft Visual Modeller Petal file (ASCII) | ptl |  |
| linguist/849523096 | Microsoft Visual Studio Solution | .sln |  |
| fmt/385 | Microsoft Windows Cursor | cur | image/x-win-bitmap |
| fmt/344 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| fmt/345 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| x-fmt/153 | Microsoft Windows Enhanced Metafile | emf | image/emf |
| fmt/971 | Microsoft Windows Movie Maker File | mswmm |  |
| x-fmt/428 | Microsoft Windows Shortcut | lnk |  |
| fmt/609 | Microsoft Word (Generic) | doc | application/msword |
| fmt/754 | Microsoft Word Document (Password Protected) | wbk, doc | application/msword |
| fmt/755 | Microsoft Word Document Template (Password Protected) | dot | application/msword |
| x-fmt/45 | Microsoft Word Document Template | dot |  |
| fmt/39 | Microsoft Word Document | doc | application/msword |
| fmt/40 | Microsoft Word Document | doc, wbk | application/msword |
| fmt/346 | Microsoft Word for Macintosh Document | mcw | application/msword |
| x-fmt/1 | Microsoft Word for Macintosh Document | mcw | application/msword |
| x-fmt/129 | Microsoft Word for Macintosh Document |  |  |
| x-fmt/2 | Microsoft Word for Macintosh Document |  |  |
| x-fmt/64 | Microsoft Word for Macintosh Document | mcw | application/msword |
| x-fmt/65 | Microsoft Word for Macintosh Document | mcw | application/msword |
| fmt/1688 | Microsoft Word for MS-DOS Document | doc | application/msword |
| x-fmt/273 | Microsoft Word for MS-DOS Document |  |  |
| x-fmt/274 | Microsoft Word for MS-DOS Document | doc | application/msword |
| x-fmt/275 | Microsoft Word for MS-DOS Document | doc | application/msword |
| x-fmt/276 | Microsoft Word for MS-DOS Document | doc | application/msword |
| fmt/1689 | Microsoft Word for MS-DOS Glossary File | gly | application/msword |
| fmt/1691 | Microsoft Word for MS-DOS Printer Description File | prd | application/msword |
| fmt/1690 | Microsoft Word for MS-DOS Style Sheet File | sty | application/msword |
| fmt/37 | Microsoft Word for Windows Document | doc | application/msword |
| fmt/38 | Microsoft Word for Windows Document | doc | application/msword |
| x-fmt/204 | Microsoft Word for Windows Macro | wpm |  |
| fmt/412 | Microsoft Word for Windows | docx, wbk | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| fmt/599 | Microsoft Word Macro-Enabled Document Template | dotm | application/vnd.ms-word.template.macroEnabled.12 |
| fmt/597 | Microsoft Word Template | dotx | application/vnd.openxmlformats-officedocument.wordprocessingml.template |
| fmt/169 | Microsoft Works Database for DOS | wdb |  |
| fmt/170 | Microsoft Works Database for DOS | wdb |  |
| fmt/171 | Microsoft Works Database for DOS | wdb |  |
| fmt/259 | Microsoft Works Database for DOS | wdb |  |
| fmt/260 | Microsoft Works Database for DOS | wdb |  |
| fmt/261 | Microsoft Works Database for DOS | wdb |  |
| fmt/268 | Microsoft Works Database for Macintosh | wdb |  |
| fmt/269 | Microsoft Works Database for Macintosh | wdb |  |
| fmt/219 | Microsoft Works Database for Windows | wdb |  |
| fmt/222 | Microsoft Works Database for Windows | wdb |  |
| fmt/223 | Microsoft Works Database for Windows | wdb |  |
| fmt/224 | Microsoft Works Database for Windows | wdb |  |
| fmt/225 | Microsoft Works Database for Windows | wdb |  |
| fmt/226 | Microsoft Works Database for Windows | wdb |  |
| fmt/246 | Microsoft Works Database for Windows | wdb |  |
| fmt/249 | Microsoft Works Database for Windows | wdb |  |
| fmt/252 | Microsoft Works Database for Windows | wdb |  |
| fmt/256 | Microsoft Works Database for Windows | wdb |  |
| x-fmt/344 | Microsoft Works Database | bdb |  |
| x-fmt/345 | Microsoft Works Document | bps |  |
| x-fmt/120 | Microsoft Works for Windows |  |  |
| fmt/167 | Microsoft Works Spreadsheet for DOS |  |  |
| fmt/168 | Microsoft Works Spreadsheet for DOS |  |  |
| fmt/262 | Microsoft Works Spreadsheet for DOS |  |  |
| fmt/263 | Microsoft Works Spreadsheet for DOS |  |  |
| fmt/264 | Microsoft Works Spreadsheet for DOS |  |  |
| fmt/270 | Microsoft Works Spreadsheet for Macintosh | wks |  |
| fmt/271 | Microsoft Works Spreadsheet for Macintosh | wks |  |
| fmt/220 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/227 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/228 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/229 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/230 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/231 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/247 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/250 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/253 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/257 | Microsoft Works Spreadsheet for Windows |  |  |
| fmt/166 | Microsoft Works Spreadsheet | wks |  |
| fmt/901 | Microsoft Works Spreadsheet | xlr |  |
| x-fmt/118 | Microsoft Works Spreadsheet |  |  |
| fmt/163 | Microsoft Works Word Processor 1-3 for DOS and 2 for Windows | wps |  |
| fmt/233 | Microsoft Works Word Processor 3-4 for Windows | wps |  |
| fmt/258 | Microsoft Works Word Processor 5-6 | wps |  |
| fmt/265 | Microsoft Works Word Processor DOS |  |  |
| fmt/266 | Microsoft Works Word Processor DOS |  |  |
| fmt/267 | Microsoft Works Word Processor DOS |  |  |
| fmt/164 | Microsoft Works Word Processor for DOS |  |  |
| fmt/165 | Microsoft Works Word Processor for DOS |  |  |
| fmt/221 | Microsoft Works Word Processor for Windows |  |  |
| fmt/232 | Microsoft Works Word Processor for Windows |  |  |
| fmt/234 | Microsoft Works Word Processor for Windows |  |  |
| fmt/235 | Microsoft Works Word Processor for Windows |  |  |
| fmt/236 | Microsoft Works Word Processor for Windows |  |  |
| fmt/272 | Microsoft Works Word Processor Macintosh | wps |  |
| fmt/273 | Microsoft Works Word Processor Macintosh | wps |  |
| fmt/248 | Microsoft Works Word Processor Windows |  |  |
| fmt/251 | Microsoft Works Word Processor Windows |  |  |
| fmt/254 | Microsoft Works Word Processor Windows |  |  |
| fmt/923 | Microsoft xWMA | xwma |  |
| fmt/1358 | MicroStation Base File | bse |  |
| x-fmt/346 | Microstation CAD Drawing | dgn |  |
| fmt/1177 | MicroStation Material Library | mat |  |
| fmt/1183 | MicroStation Material Palette | pal |  |
| fmt/1626 | MicroStation Symbology Resource File | rsc |  |
| x-fmt/230 | MIDI Audio | mid, midi | audio/midi |
| apache-httpd/11881934563155833868 | mie | mie | application/x-mie |
| fmt/1470 | MIG Graphics File | mig |  |
| fmt/950 | MIME Email | eml | message/rfc822 |
| fmt/1138 | MiniCAD/VectorWorks | mcd, vwx | application/vnd.vectorworks |
| fmt/1136 | MiniCAD | mcd | application/vnd.vectorworks |
| fmt/1137 | MiniCAD | mcd | application/vnd.vectorworks |
| linguist/231 | MiniD | .minid |  |
| fmt/1431 | Minitab Portable Worksheet | mtp |  |
| fmt/1434 | Minitab Project | mpj |  |
| fmt/1436 | Minitab Project | mpj |  |
| fmt/1438 | Minitab Project | mpj |  |
| fmt/1430 | Minitab Worksheet | mtw |  |
| fmt/1432 | Minitab Worksheet | mtw |  |
| fmt/1433 | Minitab Worksheet | mtw |  |
| fmt/1435 | Minitab Worksheet | mtw |  |
| fmt/1437 | Minitab Worksheet | mtw |  |
| linguist/4896465 | MiniYAML | .yaml, .yml | text/x-yaml |
| fmt/669 | Minolta RAW | mrw |  |
| linguist/968740319 | Mint | .mint |  |
| linguist/232 | Mirah | .druby, .duby, .mirah | text/x-ruby |
| linguist/517654727 | mIRC Script | .mrc |  |
| fmt/337 | MJ2 (Motion JPEG 2000) | mj2, mjp2 | video/mj2 |
| linguist/448253929 | MLIR | .mlir |  |
| fmt/961 | Mobile eXtensible Music Format | mxmf | audio/mobile-xmf |
| apache-httpd/17681871278090163123 | mobipocket ebook | prc, mobi | application/x-mobipocket-ebook |
| apache-httpd/3942324793998983809 | mobius daf | daf | application/vnd.mobius.daf |
| apache-httpd/12614044088270288356 | mobius dis | dis | application/vnd.mobius.dis |
| apache-httpd/17302285957713971200 | mobius mbk | mbk | application/vnd.mobius.mbk |
| apache-httpd/13588310119824942220 | mobius mqy | mqy | application/vnd.mobius.mqy |
| apache-httpd/9094943142734310647 | mobius msl | msl | application/vnd.mobius.msl |
| apache-httpd/4431472809167363624 | mobius plc | plc | application/vnd.mobius.plc |
| apache-httpd/16639777093975277442 | mobius txf | txf | application/vnd.mobius.txf |
| fmt/716 | MOD Audio Module | mod |  |
| linguist/233 | Modelica | .mo | text/x-modelica |
| apache-httpd/6423878811909784008 | mods xml | mods | application/mods+xml |
| linguist/234 | Modula-2 | .mod |  |
| linguist/564743864 | Modula-3 | .i3, .ig, .m3, .mg |  |
| linguist/235 | Module Management System | .mms, .mmk |  |
| linguist/1045019587 | Mojo | .mojo | text/x-python |
| linguist/231751931 | Monkey C | .mc | text/x-csrc |
| fmt/1086 | Monkey's Audio File | ape |  |
| linguist/236 | Monkey | .monkey, .monkey2 |  |
| linguist/237 | Moocode | .moo |  |
| linguist/181453007 | MoonBit | .mbt |  |
| linguist/238 | MoonScript | .moon |  |
| apache-httpd/17697489348673614307 | mophun application | mpn | application/vnd.mophun.application |
| apache-httpd/5827300586678435774 | mophun certificate | mpc | application/vnd.mophun.certificate |
| fmt/612 | Mork | mab, msf, dat |  |
| linguist/202937027 | Motoko | .mo |  |
| linguist/477582706 | Motorola 68K Assembly | .asm, .i, .inc, .s, .x68 |  |
| linguist/638334599 | Move | .move |  |
| fmt/1970 | MOXCEL | mxl |  |
| apache-httpd/10710046949140717356 | mozilla xul xml | xul | application/vnd.mozilla.xul+xml |
| apache-httpd/3672696470516843498 | mp21 | m21, mp21 | application/mp21 |
| apache-httpd/13859451415498689483 | mp2t | ts, m2t, m2ts, mts | video/mp2t |
| apache-httpd/13380737628036455431 | mp4 | m4a, mp4a | audio/mp4 |
| apache-httpd/3013817452668215055 | mp4 | mp4s | application/mp4 |
| x-fmt/279 | MPEG 1/2 Audio Layer 3 Streaming | m3u | audio/mpeg |
| fmt/134 | MPEG 1/2 Audio Layer 3 | mp3 | audio/mpeg |
| fmt/347 | MPEG 1/2 Audio Layer I | mp1 | audio/mpeg |
| fmt/198 | MPEG Audio Stream Layer II | mp2, mpw, mpa | audio/mpeg |
| fmt/649 | MPEG-1 Elementary Stream | mpg, mpeg, m1v |  |
| x-fmt/385 | MPEG-1 Program Stream | mpeg, mpg | video/mpeg |
| fmt/640 | MPEG-2 Elementary Stream | mpg, mpeg, m2v |  |
| x-fmt/386 | MPEG-2 Program Stream | mpeg, mpg, mod | video/mpeg |
| fmt/585 | MPEG-2 Transport Stream | m2t, ts, m2ts |  |
| fmt/199 | MPEG-4 Media File | mp4, m4v, m4a, f4v, f4a | application/mp4, video/mp4 |
| apache-httpd/11650175986876150549 | mpegurl | m3u | audio/x-mpegurl |
| apache-httpd/15681581293980520132 | mpegurl | mxu, m4u | video/vnd.mpegurl |
| linguist/426 | MQL4 | .mq4, .mqh |  |
| linguist/427 | MQL5 | .mq5, .mqh |  |
| fmt/392 | MrSID Image Format (Multi-resolution Seamless Image Database) | sid |  |
| apache-httpd/10459519546450867951 | mrsid image | sid | image/x-mrsid-image |
| apache-httpd/16627114407478665957 | ms application | application | application/x-ms-application |
| apache-httpd/11231017852761258701 | ms artgalry | cil | application/vnd.ms-artgalry |
| apache-httpd/16549734338554005100 | ms asf | asf, asx | video/x-ms-asf |
| fmt/469 | MS DOS Compression Format (KWAJ Variant) |  |  |
| apache-httpd/7193524944753159053 | ms excel addin macroenabled 12 | xlam | application/vnd.ms-excel.addin.macroenabled.12 |
| apache-httpd/17758141517366411449 | ms excel sheet binary macroenabled 12 | xlsb | application/vnd.ms-excel.sheet.binary.macroenabled.12 |
| apache-httpd/453850464475411149 | ms excel sheet macroenabled 12 | xlsm | application/vnd.ms-excel.sheet.macroenabled.12 |
| apache-httpd/12161631093412850975 | ms excel template macroenabled 12 | xltm | application/vnd.ms-excel.template.macroenabled.12 |
| apache-httpd/8850202862544404839 | ms ims | ims | application/vnd.ms-ims |
| apache-httpd/9023937269523142240 | ms lrm | lrm | application/vnd.ms-lrm |
| apache-httpd/12869456523622353063 | ms photo | wdp | image/vnd.ms-photo |
| apache-httpd/5764838913370525935 | ms pki seccat | cat | application/vnd.ms-pki.seccat |
| apache-httpd/13175711861487568318 | ms pki stl | stl | application/vnd.ms-pki.stl |
| apache-httpd/11414351502892999160 | ms playready media pya | pya | audio/vnd.ms-playready.media.pya |
| apache-httpd/3237304037782105496 | ms playready media pyv | pyv | video/vnd.ms-playready.media.pyv |
| apache-httpd/17352955355019002013 | ms powerpoint addin macroenabled 12 | ppam | application/vnd.ms-powerpoint.addin.macroenabled.12 |
| apache-httpd/2047390379536670552 | ms powerpoint presentation macroenabled 12 | pptm | application/vnd.ms-powerpoint.presentation.macroenabled.12 |
| apache-httpd/8409127412828138689 | ms powerpoint slide macroenabled 12 | sldm | application/vnd.ms-powerpoint.slide.macroenabled.12 |
| apache-httpd/6816715224970428638 | ms powerpoint slideshow macroenabled 12 | ppsm | application/vnd.ms-powerpoint.slideshow.macroenabled.12 |
| apache-httpd/11696996066490782296 | ms powerpoint template macroenabled 12 | potm | application/vnd.ms-powerpoint.template.macroenabled.12 |
| apache-httpd/17484877487305115669 | ms shortcut | lnk | application/x-ms-shortcut |
| apache-httpd/663086526965138497 | ms vob | vob | video/x-ms-vob |
| apache-httpd/16874531211968188565 | ms wax | wax | audio/x-ms-wax |
| apache-httpd/12139814779460625487 | ms wmd | wmd | application/x-ms-wmd |
| apache-httpd/17481297107178477904 | ms wmx | wmx | video/x-ms-wmx |
| apache-httpd/6444568146336275519 | ms wmz | wmz | application/x-ms-wmz |
| apache-httpd/10545300983186267140 | ms wm | wm | video/x-ms-wm |
| apache-httpd/6873454529924296618 | ms word document macroenabled 12 | docm | application/vnd.ms-word.document.macroenabled.12 |
| apache-httpd/3685976441960773084 | ms word template macroenabled 12 | dotm | application/vnd.ms-word.template.macroenabled.12 |
| apache-httpd/77905493598287429 | ms works | wps, wks, wcm, wdb | application/vnd.ms-works |
| apache-httpd/10712924823320355462 | ms wvx | wvx | video/x-ms-wvx |
| apache-httpd/14888301265473117000 | ms xbap | xbap | application/x-ms-xbap |
| apache-httpd/13793382431961437112 | ms xpsdocument | xps | application/vnd.ms-xpsdocument |
| fmt/462 | MS-DOS Compression Format (SZDD Variant) |  |  |
| x-fmt/409 | MS-DOS Executable | exe |  |
| x-fmt/130 | MS-DOS Text File with line breaks |  | text/plain |
| x-fmt/15 | MS-DOS Text File |  | text/plain |
| apache-httpd/5216488401283368545 | msaccess | mdb | application/x-msaccess |
| apache-httpd/12151911956262664389 | msbinder | obd | application/x-msbinder |
| apache-httpd/5622317916550356859 | mscardfile | crd | application/x-mscardfile |
| apache-httpd/7535922962243314868 | msclip | clp | application/x-msclip |
| apache-httpd/9712560292327597037 | msdownload | exe, dll, com, bat, msi | application/x-msdownload |
| apache-httpd/4781651560126321324 | mseq | mseq | application/vnd.mseq |
| apache-httpd/10209449085056580529 | msmediaview | mvb, m13, m14 | application/x-msmediaview |
| apache-httpd/4486161733139916318 | msmetafile | wmf, wmz, emf, emz | application/x-msmetafile |
| apache-httpd/940430676671419282 | msmoney | mny | application/x-msmoney |
| apache-httpd/11706933179567706948 | msschedule | scd | application/x-msschedule |
| apache-httpd/17488117191040683567 | msterminal | trm | application/x-msterminal |
| apache-httpd/368739558323144978 | mswrite | wri | application/x-mswrite |
| linguist/218 | MTML | .mtml | text/html |
| linguist/219 | MUF | .muf, .m | text/x-forth |
| fmt/1471 | Multi Palette Picture File | mpp |  |
| fmt/1468 | multiArtist File | mg1, mg2, mg4, mg8 |  |
| x-fmt/347 | MultiMate Text File | dox, fnx, pat |  |
| fmt/1800 | Multimedia Viewer Book | mvb |  |
| x-fmt/348 | Multipage Zsoft Paintbrush Bitmap Graphics | dcx | image/x-dcx |
| fmt/528 | Multiple-image Network Graphics | mng | video/x-mng |
| fmt/719 | MultiTracker Module | mtm |  |
| linguist/416 | mupad | .mu |  |
| linguist/474864066 | Muse | .muse |  |
| fmt/965 | Music Encoding Initiative | mei |  |
| apache-httpd/14549417581595720487 | musician | mus | application/vnd.musician |
| fmt/896 | MusicXML | xml, musicxml | application/vnd.recordare.musicxml+xml |
| linguist/638334590 | Mustache | .mustache | text/x-smarty |
| fmt/1386 | Muvee autoProducer Project File | mve |  |
| fmt/1387 | Muvee autoProducer Project File | mvex |  |
| fmt/1388 | Muvee Reveal Project File | rvl |  |
| apache-httpd/6940775733493568946 | muvee style | msty | application/vnd.muvee.style |
| linguist/239 | Myghty | .myt |  |
| fmt/1197 | MyISAM Indexes File | myi |  |
| apache-httpd/3675306534347999425 | mynfc | taglet | application/vnd.mynfc |
| fmt/868 | MySQL Table Definition Format | frm |  |
| linguist/214 | M | .mumps, .m | text/x-mumps |
| linguist/775996197 | nanorc | .nanorc |  |
| x-fmt/163 | NAP Metafile | nap |  |
| linguist/178322513 | Nasal | .nas |  |
| linguist/171666519 | NASL | .nasl, .inc |  |
| fmt/364 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| fmt/365 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| fmt/366 | National Imagery Transmission Format | ntf | application/vnd.nitf |
| fmt/857 | Navisworks Document | nwd, nwc |  |
| fmt/858 | Navisworks Document | nwd, nwc |  |
| fmt/859 | Navisworks Document | nwd, nwc |  |
| fmt/860 | Navisworks Document | nwd, nwc |  |
| fmt/1280 | NCH Dictation Audio File | dct |  |
| linguist/240 | NCL | .ncl |  |
| linguist/521429430 | Nearley | .ne, .nearley |  |
| fmt/1002 | Nearly Raw Raster Data | nrrd |  |
| fmt/1003 | Nearly Raw Raster Data | nrrd |  |
| fmt/1004 | Nearly Raw Raster Data | nrrd |  |
| fmt/1005 | Nearly Raw Raster Data | nrrd |  |
| fmt/1006 | Nearly Raw Raster Data | nrrd |  |
| fmt/1963 | NEC Thermo Tracer Image File | tmp |  |
| linguist/243 | Nemerle | .n |  |
| fmt/1545 | NeoDesk Icon File | nic |  |
| fmt/1540 | NeoDisk Icon File | nic |  |
| linguist/481192983 | NEON | .neon |  |
| fmt/1743 | Nero Burning ROM Image File | nrg |  |
| fmt/1368 | Nero CoverDesigner File | ncd |  |
| linguist/417 | nesC | .nc |  |
| apache-httpd/6068593002644094061 | net fpx | npx | image/vnd.net-fpx |
| fmt/283 | netCDF-3 64-bit | nc, cdf | application/netcdf, application/x-netcdf |
| fmt/282 | netCDF-3 Classic | nc, cdf | application/netcdf, application/x-netcdf |
| linguist/245 | NetLinx+ERB | .axs.erb, .axi.erb |  |
| linguist/244 | NetLinx | .axs, .axi |  |
| linguist/246 | NetLogo | .nlogo | text/x-common-lisp |
| fmt/1132 | Netscape Bookmark File Format | htm, html |  |
| fmt/1551 | NetWare Loadable Module | nlm |  |
| apache-httpd/15808064807033990788 | neurolanguage nlu | nlu | application/vnd.neurolanguage.nlu |
| linguist/247 | NewLisp | .nl, .lisp, .lsp | text/x-common-lisp |
| x-fmt/196 | NeXt Sound |  |  |
| x-fmt/139 | NeXT/Sun sound | au | audio/basic |
| linguist/506780613 | Nextflow | .nf |  |
| apache-httpd/17070028980895976996 | nfo | nfo | text/x-nfo |
| linguist/248 | Nginx | .nginx, .nginxconf, .vhost | text/x-nginx-conf |
| fmt/983 | NIB File Format | nib |  |
| fmt/202 | Nikon Digital SLR Camera Raw Image File | nef, nrw |  |
| linguist/249 | Nim | .nim, .nim.cfg, .nimble, .nimrod, .nims |  |
| linguist/250 | Ninja | .ninja |  |
| fmt/1166 | Niton Data Transfer | ndt |  |
| linguist/251 | Nit | .nit |  |
| linguist/252 | Nix | .nix |  |
| linguist/241 | NL | .nl |  |
| linguist/136456478 | NMODL | .mod |  |
| fmt/1228 | NMRPipe | dat, pipe, ft2, ft3 |  |
| fmt/1227 | NMRView | nv |  |
| apache-httpd/8715266759443811523 | noblenet directory | nnd | application/vnd.noblenet-directory |
| apache-httpd/16428619282818802010 | noblenet sealer | nns | application/vnd.noblenet-sealer |
| apache-httpd/16923941215688082693 | noblenet web | nnw | application/vnd.noblenet-web |
| linguist/813068465 | Noir | .nr | text/x-rustsrc |
| apache-httpd/14471098247504845837 | nokia n gage data | ngdat | application/vnd.nokia.n-gage.data |
| apache-httpd/3421485305706899045 | nokia n gage symbian install | n-gage | application/vnd.nokia.n-gage.symbian.install |
| fmt/1896 | Nokia Picture Message | npm |  |
| apache-httpd/5985527066767386131 | nokia radio presets | rpss | application/vnd.nokia.radio-presets |
| apache-httpd/14018862107975690802 | nokia radio preset | rpst | application/vnd.nokia.radio-preset |
| fmt/1902 | Norton Change Directory Persistent Cache File | ncd |  |
| x-fmt/349 | Nota Bene Text File | nb |  |
| fmt/974 | Notation Interchange File Format | nif | application/vnd.music-niff |
| fmt/873 | Notation3 | n3 | text/n3 |
| apache-httpd/1042982873375166231 | novadigm edm | edm | application/vnd.novadigm.edm |
| apache-httpd/1300003046207835122 | novadigm edx | edx | application/vnd.novadigm.edx |
| apache-httpd/12498182002620875355 | novadigm ext | ext | application/vnd.novadigm.ext |
| fmt/1486 | Novell Address Book | nab |  |
| linguist/685022663 | NPM Config |  |  |
| linguist/242 | NSIS | .nsi, .nsh | text/x-nsis |
| fmt/1493 | NTI JewelCase Maker | jwc |  |
| apache-httpd/13130956661305356843 | nuera ecelp4800 | ecelp4800 | audio/vnd.nuera.ecelp4800 |
| apache-httpd/11322486937063478154 | nuera ecelp7470 | ecelp7470 | audio/vnd.nuera.ecelp7470 |
| apache-httpd/7473810765811430167 | nuera ecelp9600 | ecelp9600 | audio/vnd.nuera.ecelp9600 |
| fmt/850 | NuFile Exchange Archival Library | shk, sdk, bxy |  |
| fmt/644 | Nullsoft Scriptable Install System | nsi |  |
| fmt/1176 | Nullsoft Streaming Video | nsv |  |
| linguist/254 | NumPy | .numpy, .numpyw, .numsc | text/x-python |
| linguist/461856962 | Nunjucks | .njk |  |
| linguist/446573572 | Nushell | .nu | text/x-sh |
| fmt/816 | NUT Open Container Format | nut |  |
| linguist/253 | Nu | .nu | text/x-scheme |
| linguist/731233819 | NWScript | .nss | text/x-csrc |
| apache-httpd/15382676714485689448 | nzb | nzb | application/x-nzb |
| apache-httpd/9903176183460908302 | oasis opendocument chart template | otc | application/vnd.oasis.opendocument.chart-template |
| apache-httpd/6962085545806922702 | oasis opendocument chart | odc | application/vnd.oasis.opendocument.chart |
| apache-httpd/17995198403821469367 | oasis opendocument database | odb | application/vnd.oasis.opendocument.database |
| apache-httpd/13471152099293494062 | oasis opendocument formula template | odft | application/vnd.oasis.opendocument.formula-template |
| apache-httpd/4847972083146275462 | oasis opendocument formula | odf | application/vnd.oasis.opendocument.formula |
| apache-httpd/7221080215284733600 | oasis opendocument graphics template | otg | application/vnd.oasis.opendocument.graphics-template |
| apache-httpd/7696442023566449237 | oasis opendocument image template | oti | application/vnd.oasis.opendocument.image-template |
| apache-httpd/7212275662721677375 | oasis opendocument image | odi | application/vnd.oasis.opendocument.image |
| apache-httpd/3214923385420216598 | oasis opendocument presentation template | otp | application/vnd.oasis.opendocument.presentation-template |
| apache-httpd/13058603825924308571 | oasis opendocument spreadsheet template | ots | application/vnd.oasis.opendocument.spreadsheet-template |
| apache-httpd/16625459860859088264 | oasis opendocument text master | odm | application/vnd.oasis.opendocument.text-master |
| apache-httpd/11169527702873022564 | oasis opendocument text template | ott | application/vnd.oasis.opendocument.text-template |
| apache-httpd/14133275647000193392 | oasis opendocument text web | oth | application/vnd.oasis.opendocument.text-web |
| linguist/834374816 | OASv2-json | .json | application/json |
| linguist/105187618 | OASv2-yaml | .yaml, .yml | text/x-yaml |
| linguist/980062566 | OASv3-json | .json | application/json |
| linguist/51239111 | OASv3-yaml | .yaml, .yml | text/x-yaml |
| linguist/677210597 | Oberon | .ob2 |  |
| linguist/256 | ObjDump | .objdump |  |
| linguist/985227236 | Object Data Instance Notation | .odin |  |
| linguist/258 | Objective-C++ | .mm | text/x-objectivec |
| linguist/257 | Objective-C | .m, .h | text/x-objectivec |
| linguist/259 | Objective-J | .j, .sj |  |
| linguist/202735509 | ObjectScript | .cls |  |
| fmt/1681 | OBO Flat File Format | obo |  |
| fmt/207 | Obsidium Project File | opf |  |
| linguist/255 | OCaml | .ml, .eliom, .eliomi, .ml4, .mli, .mll, .mly | text/x-ocaml |
| apache-httpd/16088873498976028686 | octet stream | bin, dms, lrf, mar, so, dist, distz, pkg, bpk, dump, elc, deploy | application/octet-stream |
| apache-httpd/15645786132478526949 | oda | oda | application/oda |
| linguist/889244082 | Odin | .odin |  |
| apache-httpd/8488995837589784861 | oebps package xml | opf | application/oebps-package+xml |
| fmt/1700 | OGC GeoPackage | gpkg | application/geopackage+sqlite3 |
| fmt/947 | Ogg FLAC Compressed Multimedia File | ogg | audio/ogg |
| fmt/944 | Ogg Multimedia Container | ogg, ogv, spx, opus | application/ogg |
| fmt/946 | Ogg Opus Codec Compressed Multimedia File | ogg, opus | audio/ogg, audio/opus |
| fmt/948 | Ogg Speex Codec Multimedia File | ogg, spx | audio/ogg, audio/speex |
| fmt/945 | Ogg Theora Video | ogv, ogg | video/ogg |
| fmt/203 | Ogg Vorbis Codec Compressed Multimedia File | ogg | audio/ogg |
| apache-httpd/701141407755406764 | ogg | ogx | application/ogg |
| fmt/1048 | OGR GFS File | gfs |  |
| fmt/1188 | Ogre Mesh 1.x | mesh |  |
| fmt/1189 | Ogre Mesh XML | xml |  |
| fmt/722 | Oktalyzer Audio file | okt |  |
| fmt/111 | OLE2 Compound Document Format |  |  |
| apache-httpd/13950687050277105287 | olpc sugar | xo | application/vnd.olpc-sugar |
| fmt/668 | Olympus RAW | orf |  |
| apache-httpd/3732919740460660496 | oma dd2 xml | dd2 | application/vnd.oma.dd2+xml |
| apache-httpd/15288519035999881865 | omdoc xml | omdoc | application/omdoc+xml |
| linguist/260 | Omgrofl | .omgrofl |  |
| linguist/664100008 | omnetpp-msg | .msg |  |
| linguist/924868392 | omnetpp-ned | .ned |  |
| fmt/963 | OMNIC Spectral Data File | spa |  |
| fmt/1372 | OmniPage Document | opd |  |
| fmt/1373 | OmniPage Document | opd |  |
| fmt/1371 | OmniPage Pro Document | opd |  |
| x-fmt/350 | OmniPage Pro Document | met |  |
| apache-httpd/13615448736560673113 | onenote | onetoc, onetoc2, onetmp, onepkg | application/onenote |
| x-fmt/3 | Online Description Tool Format | odt |  |
| linguist/418 | ooc | .ooc |  |
| linguist/262 | Opal | .opal |  |
| linguist/261 | Opa | .opa |  |
| fmt/1889 | Open Access III Document | ext |  |
| fmt/309 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| fmt/310 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| fmt/311 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| fmt/312 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| fmt/313 | Open Financial Exchange | ofx, qfx | application/x-ofx |
| fmt/832 | Open Inventor File Format | iv |  |
| fmt/833 | Open Inventor File Format | iv |  |
| fmt/1854 | Open Media Framework Interchange | omf |  |
| fmt/1855 | Open Media Framework Interchange | omf |  |
| linguist/840483232 | Open Policy Agent | .rego |  |
| fmt/522 | Open Project File | pod |  |
| fmt/657 | Open XML Paper Specification | xps, oxps | application/oxps |
| linguist/848295328 | OpenAPI Specification v2 |  |  |
| linguist/557959099 | OpenAPI Specification v3 |  |  |
| linguist/263 | OpenCL | .cl, .opencl | text/x-csrc |
| fmt/140 | OpenDocument Database Format | odb |  |
| fmt/1752 | OpenDocument Database Format | odb | application/vnd.oasis.opendocument.base |
| fmt/424 | OpenDocument Database Format | odb |  |
| fmt/444 | OpenDocument Database Format | odb |  |
| fmt/135 | OpenDocument Format |  |  |
| fmt/139 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| fmt/1753 | OpenDocument Graphics | odg | application/vnd.oasis.opendocument.graphics |
| fmt/296 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| fmt/297 | OpenDocument Graphics | odg, otg | application/vnd.oasis.opendocument.graphics |
| fmt/138 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| fmt/1754 | OpenDocument Presentation | odp | application/vnd.oasis.opendocument.presentation |
| fmt/292 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| fmt/293 | OpenDocument Presentation | odp, otp | application/vnd.oasis.opendocument.presentation |
| fmt/137 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| fmt/1755 | OpenDocument Spreadsheet | ods | application/vnd.oasis.opendocument.spreadsheet |
| fmt/294 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| fmt/295 | OpenDocument Spreadsheet | ods, ots | application/vnd.oasis.opendocument.spreadsheet |
| fmt/136 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| fmt/1756 | OpenDocument Text | odt | application/vnd.oasis.opendocument.text |
| fmt/290 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| fmt/291 | OpenDocument Text | odt, ott | application/vnd.oasis.opendocument.text |
| linguist/264 | OpenEdge ABL | .p, .cls, .w |  |
| fmt/1001 | OpenEXR | exr | image/x-exr |
| fmt/129 | OpenOffice Calc | sxc | application/vnd.sun.xml.calc |
| fmt/127 | OpenOffice Draw | sxd | application/vnd.sun.xml.draw |
| fmt/130 | OpenOffice Impress | sxi | application/vnd.sun.xml.impress |
| fmt/128 | OpenOffice Writer | sxw | application/vnd.sun.xml.writer |
| apache-httpd/9058684993375571790 | openofficeorg extension | oxt | application/vnd.openofficeorg.extension |
| linguist/153739399 | OpenQASM | .qasm |  |
| fmt/998 | OpenRaster Image Format | ora | image/openraster |
| linguist/265 | OpenRC runscript |  | text/x-sh |
| linguist/266 | OpenSCAD | .scad |  |
| linguist/598917541 | OpenStep Property List | .plist, .glyphs |  |
| linguist/374317347 | OpenType Feature File | .fea |  |
| fmt/520 | OpenType Font File | otf | font/otf |
| fmt/1947 | OpenWayback CDXJ File Format | cdx, cdxj |  |
| apache-httpd/535144868066071015 | openxmlformats officedocument presentationml slide | sldx | application/vnd.openxmlformats-officedocument.presentationml.slide |
| fmt/1882 | OPML File | opml |  |
| fmt/1883 | OPML File | opml |  |
| apache-httpd/18287376524010136670 | opml | opml | text/x-opml |
| fmt/695 | Optimised Dalvik Executable Format | odex |  |
| linguist/723589315 | Option List |  | text/x-sh |
| fmt/1465 | OrCAD Layout File | max |  |
| fmt/1572 | OrCAD Project File | opj |  |
| fmt/1457 | OrgPlus File | opx, opxt, ops |  |
| linguist/267 | Org | .org |  |
| fmt/1123 | Origin Project Format | opj, ogg, ogm, ogw |  |
| fmt/1124 | Origin Project Format | opju, oggu, ogmu, ogwu |  |
| x-fmt/25 | OS/2 Bitmap |  |  |
| x-fmt/270 | OS/2 Bitmap | bmp | image/bmp |
| x-fmt/143 | OS/2 Change Control File | cin |  |
| x-fmt/67 | OS/2 Presentation Manager Metafile (MET) | met |  |
| apache-httpd/5150666565402240513 | osgeo mapguide package | mgp | application/vnd.osgeo.mapguide.package |
| apache-httpd/9288499656011032268 | osgi dp | dp | application/vnd.osgi.dp |
| apache-httpd/4956746657128679058 | osgi subsystem | esa | application/vnd.osgi.subsystem |
| fmt/839 | Outlook Express Folder Database | dbx |  |
| fmt/838 | Outlook Express Message Database | dbx |  |
| linguist/269 | Oxygene | .oxygene |  |
| linguist/268 | Ox | .ox, .oxh, .oxo |  |
| linguist/270 | Oz | .oz | text/x-oz |
| fmt/823 | P00 C64 Image Format | p00, p01, p02, p03, p04 |  |
| linguist/348895984 | P4 | .p4 |  |
| fmt/1721 | Pablo Paint Raster Image | ppp, pa3 |  |
| fmt/1608 | Packed-Ice True Colour Picture [Spooky Sprites] (Atari Falcon) | trp, tru |  |
| fmt/1606 | Packed-Ice True Colour Sprites [Spooky Sprites] (Atari Falcon) | trs |  |
| linguist/756774415 | Pact | .pact |  |
| fmt/876 | Pagemaker Document (Generic) | p65, pmd, pmt | application/vnd.pagemaker |
| x-fmt/351 | PageMaker Document | pm3 |  |
| fmt/1686 | PageMaker Mac Document |  | application/vnd.pagemaker |
| fmt/1687 | PageMaker Mac Document |  | application/vnd.pagemaker |
| fmt/1718 | PageMaker Mac Document | p65, t65, pmd, pmt | application/vnd.pagemaker |
| fmt/1719 | PageMaker Mac Document | pm6, pt6 | application/vnd.pagemaker |
| x-fmt/173 | PageMaker PC Document | pm5, pt5 | application/vnd.pagemaker |
| x-fmt/174 | PageMaker PC Document | pm6, pt6 | application/vnd.pagemaker |
| x-fmt/181 | PageMaker PC Document | p65, t65, pmd, pmt | application/vnd.pagemaker |
| x-fmt/352 | PageMaker PC Document | pm4, pt4 | application/vnd.pagemaker |
| x-fmt/198 | Pagemaker TableEditor Graphics | tbl |  |
| fmt/1597 | PageMaker Template File | pt5 |  |
| x-fmt/200 | PageMaker Time Stamp File | tym |  |
| fmt/348 | Paint Shop Pro Image | pspimage |  |
| fmt/349 | Paint Shop Pro Image | pspimage |  |
| x-fmt/233 | Paint Shop Pro Image | psp |  |
| x-fmt/234 | Paint Shop Pro Image | psp |  |
| x-fmt/297 | Paint Shop Pro Image | psp, pspimage |  |
| x-fmt/298 | Paint Shop Pro Image | psp, pspimage |  |
| x-fmt/376 | Paint Shop Pro Image | pspimage |  |
| x-fmt/377 | Paint Shop Pro Image | psp |  |
| x-fmt/187 | Painter RIFF Image File | rif |  |
| fmt/1733 | PaintShop Plus Compressed Format | psc, da4 |  |
| fmt/217 | PaintShop Pro Browser Cache File | jbf |  |
| fmt/1654 | Palm Database ImageViewer Format | pdb |  |
| apache-httpd/8177121348548023314 | palm | pdb, pqa, oprc | application/vnd.palm |
| fmt/662 | Panasonic Raw | rw2 |  |
| linguist/276 | Pan | .pan |  |
| fmt/1223 | PaperPort MAX | max |  |
| fmt/1224 | PaperPort MAX | max |  |
| fmt/1225 | PaperPort MAX | max |  |
| fmt/1339 | PaperPort MAX | max |  |
| fmt/1965 | Papyrus Document | pap, pav, pbf |  |
| linguist/277 | Papyrus | .psc |  |
| x-fmt/307 | Paradox Database Memo Field (Binary Large Object) | dbq, mb |  |
| fmt/350 | Paradox Database Table | db |  |
| fmt/351 | Paradox Database Table | db |  |
| fmt/352 | Paradox Database Table | db |  |
| x-fmt/147 | Paradox Database Table | db |  |
| linguist/279 | Parrot Assembly | .pasm |  |
| linguist/280 | Parrot Internal Representation | .pir |  |
| linguist/278 | Parrot | .parrot |  |
| fmt/1619 | Pascal Source Code | pas |  |
| apache-httpd/4555007942525437763 | pascal | p, pas | text/x-pascal |
| linguist/281 | Pascal | .pas, .dfm, .dpr, .inc, .lpr, .pascal, .pp | text/x-pascal |
| fmt/1904 | Pasti Floppy Disk Image | stx |  |
| apache-httpd/11260398242129167003 | patch ops error xml | xer | application/patch-ops-error+xml |
| apache-httpd/5060715562509657656 | pawaafile | paw | application/vnd.pawaafile |
| linguist/271 | Pawn | .pwn, .inc, .sma |  |
| x-fmt/170 | PC Paint Bitmap | pic |  |
| fmt/780 | pcap Next Generation Packet Capture | pcapng | application/vnd.tcpdump.pcap |
| fmt/779 | pcap Packet Capture | pcap, cap, dmp | application/vnd.tcpdump.pcap |
| fmt/1936 | PCRaster | csf, map |  |
| apache-httpd/1383629243372609493 | pcx | pcx | image/x-pcx |
| fmt/86 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| fmt/87 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| fmt/88 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| fmt/89 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| fmt/90 | PCX | pcx, pcc | image/vnd.zbrush.pcx |
| linguist/736235603 | PDDL | .pddl |  |
| fmt/1129 | PDF 2.0 - Portable Document Format | pdf | application/pdf |
| fmt/1451 | PDF Portfolio | pdf |  |
| fmt/1095 | PEA Archive Format | pea |  |
| fmt/330 | Peak Graphical Waveform File | pk |  |
| fmt/1952 | PechaMaker Format | pxp |  |
| linguist/81442128 | PEG.js | .pegjs, .peggy | text/javascript |
| fmt/1781 | Pentax PEF Image File | pef | image/dng |
| linguist/840372442 | Pep8 | .pep |  |
| fmt/1898 | Perfect ZX Tape (PZX) Image Format | pzx |  |
| fmt/870 | Perl Script | pl |  |
| linguist/282 | Perl | .pl, .al, .cgi, .fcgi, .perl, .ph, .plx, .pm, .psgi, .t | text/x-perl |
| fmt/854 | Personal Ancestral File (PAF) | paf |  |
| fmt/855 | Personal Ancestral File (PAF) | paf |  |
| fmt/856 | Personal Ancestral File (PAF) | paf |  |
| fmt/1710 | Persuasion Auto-Template Interchange File | atf |  |
| fmt/1701 | Persuasion Mac Document | pr1 |  |
| fmt/1702 | Persuasion Mac Document | pr2 |  |
| fmt/1703 | Persuasion Mac Document | pr2 |  |
| fmt/1704 | Persuasion Mac Document | pr3 |  |
| fmt/1705 | Persuasion Mac Document | pn4 |  |
| fmt/1708 | Persuasion Player File | ppf |  |
| fmt/1709 | Persuasion Presentation Interchange File | prf |  |
| fmt/1706 | Persuasion Windows Document | pr2, at2 |  |
| fmt/1707 | Persuasion Windows Document | pr3, at3, pn4, at4 |  |
| fmt/1284 | PFS:First Choice Database | fol |  |
| fmt/1282 | PFS:First Choice Document | doc |  |
| fmt/1283 | PFS:First Choice Document | doc |  |
| fmt/1285 | PFS:First Choice Graph | gra |  |
| fmt/1414 | PFS:Write Document | pfs |  |
| apache-httpd/3445346377631037340 | pg format | str | application/vnd.pg.format |
| apache-httpd/11105274837093552377 | pg osasli | ei6 | application/vnd.pg.osasli |
| apache-httpd/13565162211660515468 | pgp encrypted | pgp | application/pgp-encrypted |
| apache-httpd/10735678142131287808 | pgp signature | asc, sig | application/pgp-signature |
| fmt/1489 | Phantom CINE Compressed Video File | cci |  |
| fmt/1488 | Phantom CINE Video File | cine, cin |  |
| fmt/1061 | Phase One IIQ Raw Image | iiq |  |
| fmt/1060 | Phase One Raw Image | cap, capture |  |
| fmt/667 | Photoshop Curve File | acv, atf |  |
| x-fmt/169 | PHP Script Page | php | text/html |
| linguist/272 | PHP | .php, .aw, .ctp, .fcgi, .inc, .php3, .php4, .php5, .phps, .phpt | application/x-httpd-php |
| linguist/284 | Pickle | .pkl |  |
| linguist/285 | PicoLisp | .l |  |
| x-fmt/166 | PICS Animation | pcs |  |
| apache-httpd/13788567070806270645 | pics rules | prf | application/pics-rules |
| apache-httpd/18162782278582436098 | picsel | efif | application/vnd.picsel |
| fmt/1360 | Picture Publisher Bitmap | ppf |  |
| x-fmt/176 | Picture Publisher Bitmap | pp4 |  |
| x-fmt/85 | Picture Publisher Bitmap | pp5 |  |
| linguist/425 | Pic | .pic, .chem | text/troff |
| linguist/286 | PigLatin | .pig |  |
| linguist/287 | Pike | .pike, .pmod |  |
| linguist/684385621 | Pip Requirements |  |  |
| fmt/1745 | PixArt Bitmap | pix |  |
| fmt/670 | PKCS #7 Cryptographic Message File | p7m, p7b, p7s | application/pkcs7-mime, application/pkcs7-signature |
| apache-httpd/147689487279366576 | pkcs10 | p10 | application/pkcs10 |
| apache-httpd/17536636616578527298 | pkcs12 | p12, pfx | application/x-pkcs12 |
| apache-httpd/10927828397113750156 | pkcs7 certificates | p7b, spc | application/x-pkcs7-certificates |
| apache-httpd/12396323594280000532 | pkcs7 certreqresp | p7r | application/x-pkcs7-certreqresp |
| apache-httpd/8480341370012650784 | pkcs8 | p8 | application/pkcs8 |
| apache-httpd/4889268195384677215 | pkix attr cert | ac | application/pkix-attr-cert |
| apache-httpd/13433049222230317362 | pkix cert | cer | application/pkix-cert |
| apache-httpd/17267006425445237121 | pkix crl | crl | application/pkix-crl |
| apache-httpd/6318308104775159433 | pkix pkipath | pkipath | application/pkix-pkipath |
| apache-httpd/992198684389833801 | pkixcmp | pki | application/pkixcmp |
| linguist/288822799 | Pkl | .pkl |  |
| x-fmt/111 | Plain Text File | txt | text/plain |
| linguist/833504686 | PlantUML | .puml, .iuml, .plantuml |  |
| fmt/314 | Play SID Audio | sid, psid | audio/prs.sid |
| fmt/315 | Play SID Audio | sid, psid | audio/prs.sid |
| linguist/274 | PLpgSQL | .pgsql, .sql | text/x-sql |
| apache-httpd/8533502177668954884 | pls xml | pls | application/pls+xml |
| linguist/273 | PLSQL | .pls, .bdy, .ddl, .fnc, .pck, .pkb, .pks, .plb, .plsql, .prc, .spc, .sql, .tpb, .tps, .trg, .vw | text/x-plsql |
| apache-httpd/2471663342369167330 | pmi widget | wg | application/vnd.pmi.widget |
| apache-httpd/17710107248733465858 | pn realaudio plugin | rmp | audio/x-pn-realaudio-plugin |
| x-fmt/94 | Pocket Word Document | psw, pwd |  |
| x-fmt/96 | Pocket Word Template | pwt |  |
| apache-httpd/17072281593186703027 | pocketlearn | plf | application/vnd.pocketlearn |
| fmt/396 | PocketMobi (Palm Resource) File | mobi, prc |  |
| linguist/155357471 | Pod 6 | .pod, .pod6 |  |
| linguist/288 | Pod | .pod | text/x-perl |
| linguist/289 | PogoScript | .pogo |  |
| linguist/839112914 | Polar | .polar |  |
| fmt/831 | Polygon File Format | ply |  |
| fmt/519 | Polynomial Texture Map | ptm |  |
| linguist/290 | Pony | .pony |  |
| fmt/405 | Portable Any Map | pam |  |
| apache-httpd/10418899209645427276 | portable anymap | pnm | image/x-portable-anymap |
| x-fmt/164 | Portable Bitmap Image - ASCII | pbm | image/x-portable-bitmap |
| fmt/409 | Portable Bitmap Image - Binary | pbmb, pnm |  |
| fmt/1720 | Portable Compiled Format | pcf |  |
| fmt/1064 | Portable Database | pdb |  |
| fmt/1065 | Portable Database | pdb |  |
| fmt/1066 | Portable Database | pdb |  |
| fmt/322 | Portable Form File | pff |  |
| apache-httpd/3427479898683859874 | portable graymap | pgm | image/x-portable-graymap |
| fmt/407 | Portable Grey Map - ASCII | pgma, pgm |  |
| fmt/406 | Portable Grey Map - Binary | pgmb, pgm |  |
| fmt/11 | Portable Network Graphics | png | image/png |
| fmt/12 | Portable Network Graphics | png | image/png |
| fmt/13 | Portable Network Graphics | png | image/png |
| x-fmt/178 | Portable Pixel Map - ASCII | ppm | image/x-portable-pixmap |
| fmt/408 | Portable Pixel Map - Binary | ppm, ppmb |  |
| fmt/959 | Portable Sound Format | psf, psf1, psflib, minipsf, minipsf1, gsf, gsflib, minigsf |  |
| fmt/1734 | Portfolio Graphics Compressed File | pgc |  |
| linguist/832391833 | Portugol | .por |  |
| linguist/262764437 | PostCSS | .pcss, .postcss |  |
| x-fmt/93 | Postscript Support File | psf |  |
| fmt/501 | PostScript | ps | application/postscript |
| linguist/291 | PostScript | .ps, .eps, .epsi, .pfa |  |
| x-fmt/406 | PostScript | ps | application/postscript |
| x-fmt/407 | PostScript | ps | application/postscript |
| x-fmt/408 | PostScript | ps | application/postscript |
| x-fmt/91 | Postscript | ps | application/postscript |
| linguist/275 | POV-Ray SDL | .pov, .inc |  |
| apache-httpd/13637343208873454390 | powerbuilder6 | pbd | application/vnd.powerbuilder6 |
| linguist/292 | PowerBuilder | .pbt, .sra, .sru, .srw |  |
| fmt/1201 | PowerCADD |  |  |
| fmt/1200 | PowerDraw |  |  |
| fmt/1731 | PowerGraphics Image File | pgr |  |
| fmt/510 | PowerProject Teamplan | pdb |  |
| fmt/511 | PowerProject | pp |  |
| fmt/512 | PowerProject | pp |  |
| fmt/513 | PowerProject | pp |  |
| fmt/514 | PowerProject | pp |  |
| fmt/515 | PowerProject | pp |  |
| fmt/516 | PowerProject | pp |  |
| fmt/517 | PowerProject | pp |  |
| linguist/293 | PowerShell | .ps1, .psd1, .psm1 | application/x-powershell |
| fmt/782 | PowerVR Object Data | pod |  |
| fmt/1829 | PPTX Strict OOXML Presentation | pptx | application/vnd.openxmlformats-officedocument.presentationml.presentation |
| fmt/1164 | Praat Picture File | prapic |  |
| fmt/1165 | Praat Script File | praat |  |
| fmt/1801 | Praat TextGrid | textgrid | text/praat-textgrid |
| linguist/106029007 | Praat | .praat |  |
| fmt/1070 | Preferred Executable Format |  |  |
| apache-httpd/16902732272208064392 | previewsystems box | box | application/vnd.previewsystems.box |
| fmt/1455 | Primavera P6 Project Management XER File | xer |  |
| fmt/185 | Prime OCR | pro |  |
| fmt/183 | PrimeOCR | pro |  |
| fmt/184 | PrimeOCR | pro |  |
| fmt/186 | PrimeOCR | pro |  |
| fmt/187 | PrimeOCR | pro |  |
| fmt/188 | PrimeOCR | pro |  |
| fmt/1302 | PrintMaster Gold Project | ban, cal, car, let, sig |  |
| fmt/1732 | Prism Paint Bitmap | pnt, tpi |  |
| linguist/499933428 | Prisma | .prisma |  |
| fmt/1727 | Pro Tools Session File | ptx |  |
| fmt/1951 | Pro Tools Session File | ptf, pts |  |
| fmt/701 | Processing Development Environment | pde |  |
| linguist/294 | Processing | .pde |  |
| linguist/305313959 | Procfile |  |  |
| x-fmt/353 | Professional Write Text File | pw |  |
| fmt/1957 | Program Embroidery Stitch (PES) File | pes |  |
| fmt/1128 | Progressive Graphics File | pgf |  |
| linguist/716513858 | Proguard | .pro |  |
| linguist/295 | Prolog | .pl, .plt, .pro, .prolog, .yap |  |
| linguist/441858312 | Promela | .pml |  |
| linguist/296 | Propeller Spin | .spin |  |
| fmt/2009 | Protein Data Bank File | pdb |  |
| apache-httpd/17614153492566206582 | proteus magazine | mgz | application/vnd.proteus.magazine |
| linguist/436568854 | Protocol Buffer Text Format | .textproto, .pbt, .pbtxt |  |
| linguist/297 | Protocol Buffer | .proto | text/x-protobuf |
| apache-httpd/3404436964465585248 | prs btif | btif | image/prs.btif |
| apache-httpd/4518020368328841134 | prs cww | cww | application/prs.cww |
| apache-httpd/18042305944542558408 | prs lines tag | dsc | text/prs.lines.tag |
| fmt/1744 | Psion Series 3 Bitmap | pic |  |
| apache-httpd/17622281248786972968 | pskc xml | pskcxml | application/pskc+xml |
| fmt/1897 | Ptex File Format | ptx |  |
| fmt/1343 | PTGui Project File | pts |  |
| fmt/1344 | PTGui Project File | pts |  |
| linguist/298 | Public Key | .asc, .pub | application/pgp |
| apache-httpd/7428950758947074420 | publishare delta tree | qps | application/vnd.publishare-delta-tree |
| linguist/179 | Pug | .jade, .pug | text/x-pug |
| fmt/360 | pulse EKKO data file | dt1 |  |
| fmt/361 | pulse EKKO header file | hd |  |
| linguist/299 | Puppet | .pp | text/x-puppet |
| linguist/300 | Pure Data | .pd |  |
| linguist/301 | PureBasic | .pb, .pbi |  |
| linguist/302 | PureScript | .purs | text/x-haskell |
| apache-httpd/18177019580753205670 | pvi ptid1 | ptid | application/vnd.pvi.ptid1 |
| linguist/252961827 | Pyret | .arr |  |
| fmt/1106 | Python Compiled File | pyc |  |
| fmt/1107 | Python Compiled File | pyc |  |
| fmt/1108 | Python Compiled File | pyc |  |
| fmt/1109 | Python Compiled File | pyc |  |
| fmt/1110 | Python Compiled File | pyc |  |
| fmt/1111 | Python Compiled File | pyc |  |
| fmt/1112 | Python Compiled File | pyc |  |
| fmt/1113 | Python Compiled File | pyc |  |
| fmt/1114 | Python Compiled File | pyc |  |
| fmt/1115 | Python Compiled File | pyc |  |
| fmt/1116 | Python Compiled File | pyc |  |
| fmt/1117 | Python Compiled File | pyc |  |
| fmt/1118 | Python Compiled File | pyc |  |
| fmt/939 | Python Compiled File | pyc |  |
| fmt/940 | Python Compiled File | pyc |  |
| linguist/428 | Python console |  |  |
| fmt/938 | Python Source Code File | py |  |
| linguist/304 | Python traceback | .pytb |  |
| linguist/303 | Python | .py, .cgi, .fcgi, .gyp, .gypi, .lmi, .py3, .pyde, .pyi, .pyp, .pyt, .pyw, .rpy, .spec, .tac, .wsgi, .xpy | text/x-python |
| linguist/697448245 | Q# | .qs |  |
| fmt/1045 | Q&A Word Processor Document |  |  |
| fmt/962 | QCP Audio File Format | qcp | audio/qcelp |
| linguist/306 | QMake | .pro, .pri |  |
| linguist/305 | QML | .qml, .qbs |  |
| fmt/830 | Qsplat Model | qs |  |
| linguist/558193693 | Qt Script | .qs | text/javascript |
| fmt/888 | QuadriSpace Format | qsd, qsl, qsm, qst |  |
| linguist/375265331 | Quake |  |  |
| apache-httpd/18202258566732886046 | quark quarkxpress | qxd, qxt, qwd, qwt, qxl, qxb | application/vnd.quark.quarkxpress |
| x-fmt/182 | QuarkXPress Data File | qxd, qxt, qxp, qcd, qxl, qxb, qwd, qwt, qpt | application/vnd.Quark.QuarkXPress |
| fmt/1317 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1318 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1319 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1320 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1442 | QuarkXPress Document |  | application/vnd.Quark.QuarkXPress |
| fmt/1443 | QuarkXPress Document |  | application/vnd.Quark.QuarkXPress |
| fmt/1444 | QuarkXPress Document | qxd, qxt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1321 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1322 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1323 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1324 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1325 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1326 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1327 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1328 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1445 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1446 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1494 | QuarkXPress Project | qxp, qpt, qwd | application/vnd.Quark.QuarkXPress |
| fmt/1495 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| fmt/2006 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| fmt/2007 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| fmt/2008 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| fmt/651 | QuarkXPress Project | qpt, qwd, qxp | application/vnd.Quark.QuarkXPress |
| fmt/652 | QuarkXPress Project | qpt, qwd, qxp | application/vnd.Quark.QuarkXPress |
| fmt/685 | QuarkXPress Project | qxp, qwd, qpt | application/vnd.Quark.QuarkXPress |
| fmt/650 | QuarkXPress Report File | qxp report, xtg, qxp%20report | application/vnd.Quark.QuarkXPress |
| fmt/508 | Quarter Inch Cartridge Host Interchange Format | qic |  |
| x-fmt/121 | Quattro Pro Spreadsheet for DOS | wq1, wkq |  |
| x-fmt/122 | Quattro Pro Spreadsheet for DOS | wq2, wkq |  |
| fmt/834 | Quattro Pro Spreadsheet for Windows | wb1 |  |
| fmt/835 | Quattro Pro Spreadsheet for Windows | wb2 |  |
| fmt/836 | Quattro Pro Spreadsheet | wb3 |  |
| fmt/837 | Quattro Pro Spreadsheet | qpw |  |
| linguist/593107205 | QuickBASIC | .bas | text/x-vb |
| fmt/1354 | QuickBooks Backup File | qbb |  |
| fmt/1049 | QuickDraw 3D Metafile (ASCII) | 3dmf |  |
| fmt/1050 | QuickDraw 3D Metafile (Binary) | 3dmf |  |
| fmt/1203 | QuickDraw 3D Metafile (Binary) | 3dmf |  |
| fmt/1861 | Quicken 3 Database File | qst |  |
| x-fmt/213 | Quicken Data File | abd, qdf, qel |  |
| fmt/308 | Quicken Data Format | qdf |  |
| fmt/307 | Quicken Interchange Format | qif | application/qif |
| x-fmt/384 | Quicktime | mov, qtm | video/quicktime |
| linguist/970539067 | q | .q |  |
| fmt/1599 | R Program File | r |  |
| linguist/316 | Racket | .rkt, .rktd, .rktl, .scrbl |  |
| fmt/591 | Radiance RGBE Image Format | hdr, pic, rgbe, xyze | image/vnd.radiance |
| linguist/317 | Ragel | .rl |  |
| fmt/1894 | RagTime Document File |  |  |
| fmt/1895 | RagTime Document File | rtd, rtt |  |
| linguist/283 | Raku | .6pl, .6pm, .nqp, .p6, .p6l, .p6m, .pl, .pl6, .pm, .pm6, .raku, .rakumod, .t | text/x-perl |
| linguist/308 | RAML | .raml | text/x-yaml |
| fmt/411 | RAR Archive | rar | application/vnd.rar |
| fmt/613 | RAR Archive | rar | application/vnd.rar |
| x-fmt/264 | RAR Archive | rar | application/vnd.rar |
| apache-httpd/16410212459293594184 | rar compressed | rar | application/x-rar-compressed |
| linguist/173616037 | Rascal | .rsc |  |
| fmt/1539 | Raster Matrix Format | rsw |  |
| x-fmt/185 | Raw Bitmap | raw |  |
| fmt/1252 | Raw Flux Image | rfi |  |
| fmt/41 | Raw JPEG Stream | jpe, jpg, jpeg, jif, jfif, jfi | image/jpeg |
| fmt/1810 | Raw PIMA SWIR Reflectance Spectral File | fos |  |
| linguist/318 | Raw token data | .raw |  |
| linguist/899227493 | RBS | .rbs | text/x-ruby |
| fmt/1198 | RData | rdata |  |
| fmt/1199 | RData | rdata |  |
| fmt/875 | RDF/XML | rdf | application/rdf+xml |
| linguist/309 | RDoc | .rdoc |  |
| linguist/538732839 | Readline Config |  |  |
| fmt/316 | Real SID Audio | sid | audio/prs.sid |
| x-fmt/277 | Real Video | rv | video/vnd.rn-realvideo |
| x-fmt/183 | RealAudio Metafile | ram | audio/vnd.rn-realaudio, audio/x-pn-realaudio |
| fmt/404 | RealAudio | ra |  |
| x-fmt/278 | RealAudio | ra | audio/vnd.rn-realaudio |
| linguist/310 | REALbasic | .rbbas, .rbfrm, .rbmnu, .rbres, .rbtbar, .rbuistate |  |
| fmt/728 | RealLegal E-Transcript | ptx |  |
| x-fmt/190 | RealMedia | rm, rmvb | application/vnd.rn-realmedia |
| fmt/204 | RealVideo Clip | rv |  |
| apache-httpd/9040307230090136548 | realvnc bed | bed | application/vnd.realvnc.bed |
| linguist/319002153 | ReasonLIGO | .religo | text/x-rustsrc |
| linguist/869538413 | Reason | .re, .rei | text/x-rustsrc |
| linguist/319 | Rebol | .reb, .r, .r2, .r3, .rebol |  |
| linguist/865765202 | Record Jar |  | text/x-properties |
| fmt/1664 | RED Thumbnail File | rtn |  |
| fmt/1039 | Redcode Metadata (RMD) File | rmd |  |
| fmt/1038 | Redcode RAW (R3D) Media File | r3d |  |
| fmt/588 | Redcode RAW (R3D) Media File | r3d |  |
| linguist/321 | Redcode | .cw |  |
| linguist/1020148948 | Redirect Rules |  |  |
| fmt/1215 | Reduced Resolution Dataset | img, ovr, rrd, aux, aoi, cff, fft, gcc, sig, sml |  |
| linguist/320 | Red | .red, .reds |  |
| apache-httpd/47924783087979020 | reginfo xml | rif | application/reginfo+xml |
| linguist/363378884 | Regular Expression | .regexp, .regex |  |
| apache-httpd/5054915061890795333 | relax ng compact syntax | rnc | application/relax-ng-compact-syntax |
| linguist/322 | Ren'Py | .rpy |  |
| linguist/323 | RenderScript | .rs, .rsh |  |
| linguist/501875647 | ReScript | .res | text/x-rustsrc |
| fmt/1886 | Resource Interchange File Format (RIFF) |  |  |
| apache-httpd/6901913721037133475 | resource lists diff xml | rld | application/resource-lists-diff+xml |
| apache-httpd/47530549795300406 | resource lists xml | rl | application/resource-lists+xml |
| fmt/1565 | reStructuredText | rst |  |
| linguist/419 | reStructuredText | .rst, .rest, .rest.txt, .rst.txt | text/x-rst |
| x-fmt/11 | Revisable-Form-Text Document Content Architecture |  |  |
| x-fmt/446 | Revit External Group | rvg | application/octet-stream |
| x-fmt/443 | Revit Family File | rfa | application/octet-stream |
| x-fmt/444 | Revit Family Template | rft | application/octet-stream |
| x-fmt/447 | Revit Project | rvt | application/octet-stream |
| x-fmt/445 | Revit Template | rte | application/octet-stream |
| x-fmt/448 | Revit Workspace | rws | application/octet-stream |
| fmt/1919 | Revolution Stack | rev, livecode |  |
| linguist/311 | REXX | .rexx, .pprx, .rex |  |
| linguist/498022874 | Rez | .r |  |
| fmt/713 | RF64 Multichannel Broadcast Wave format | wav, rf64 |  |
| fmt/712 | RF64 | wav, rf64 |  |
| fmt/1289 | RFFlow Chart | flo |  |
| fmt/1290 | RFFlow Chart | flo |  |
| fmt/1291 | RFFlow Chart | flo |  |
| apache-httpd/9581195091460280608 | rgb | rgb | image/x-rgb |
| fmt/355 | Rich Text Format | rtf | application/rtf, text/rtf |
| fmt/45 | Rich Text Format | rtf | application/rtf, text/rtf |
| fmt/46 | Rich Text Format |  |  |
| fmt/47 | Rich Text Format |  |  |
| fmt/48 | Rich Text Format |  |  |
| fmt/49 | Rich Text Format |  |  |
| fmt/50 | Rich Text Format | rtf | application/rtf, text/rtf |
| fmt/51 | Rich Text Format |  |  |
| fmt/52 | Rich Text Format | rtf | application/rtf, text/rtf |
| fmt/53 | Rich Text Format | rtf | application/rtf, text/rtf |
| fmt/969 | Rich Text Format | rtf | application/rtf |
| linguist/51601661 | Rich Text Format | .rtf |  |
| apache-httpd/14552983977238544941 | richtext | rtx | text/richtext |
| fmt/624 | RIFF Palette Format | pal |  |
| fmt/956 | RIFF-based MIDI | rmi |  |
| apache-httpd/8134742735002335236 | rig cryptonote | cryptonote | application/vnd.rig.cryptonote |
| apache-httpd/17335134041164572274 | rim cod | cod | application/vnd.rim.cod |
| linguist/431 | Ring | .ring |  |
| linguist/878396783 | Riot | .riot |  |
| apache-httpd/3181405020922865967 | rip | rip | audio/vnd.rip |
| fmt/1899 | RIS Citation | ris | application/x-research-info-systems |
| apache-httpd/9575045232419729220 | rls services xml | rs | application/rls-services+xml |
| linguist/313 | RMarkdown | .qmd, .rmd | text/x-gfm |
| apache-httpd/18248750328496013465 | rn realmedia vbr | rmvb | application/vnd.rn-realmedia-vbr |
| linguist/324 | RobotFramework | .robot, .resource |  |
| linguist/674736065 | robots.txt |  |  |
| fmt/485 | Rocket Book eBook format | rb |  |
| fmt/1746 | Rocky Interlace Picture | rip |  |
| linguist/440182480 | Roc | .roc |  |
| linguist/612669833 | Roff Manpage | .1, .1in, .1m, .1x, .2, .3, .3in, .3m, .3p, .3pm, .3qt, .3x, .4, .5, .6, .7, .8, .9, .man, .mdoc | text/troff |
| linguist/141 | Roff | .roff, .1, .1in, .1m, .1x, .2, .3, .3in, .3m, .3p, .3pm, .3qt, .3x, .4, .5, .6, .7, .8, .9, .l, .man, .mdoc, .me, .ms, .n, .nr, .rno, .tmac | text/troff |
| linguist/587855233 | RON | .ron |  |
| fmt/1338 | RootsMagic Database | rmgc |  |
| linguist/325 | Rouge | .rg | text/x-clojure |
| apache-httpd/15782473511556218270 | route66 link66 xml | link66 | application/vnd.route66.link66+xml |
| linguist/592853203 | RouterOS Script | .rsc |  |
| fmt/1670 | Roxio Audio Project File | rox |  |
| fmt/1669 | Roxio Data Project File | rox |  |
| fmt/1667 | Roxio Easy Media Creator - Classic Creator File | rcl |  |
| fmt/1666 | Roxio Easy Media Creator Layout | rcl |  |
| fmt/1668 | Roxio Easy Media Creator Layout | roxio |  |
| fmt/1644 | Roxio Label Creator Project File | jwl |  |
| fmt/1645 | Roxio Label Creator Project File | jwl |  |
| fmt/1646 | Roxio Label Creator Project File | jwl |  |
| linguist/1031374237 | RPC | .x |  |
| linguist/609977990 | RPGLE | .rpgle, .sqlrpgle |  |
| apache-httpd/12296210021193523472 | rpki ghostbusters | gbr | application/rpki-ghostbusters |
| apache-httpd/16270552558721564289 | rpki manifest | mft | application/rpki-manifest |
| apache-httpd/3224209250085143021 | rpki roa | roa | application/rpki-roa |
| fmt/793 | RPM Package Manager file | rpm, src.rpm |  |
| fmt/794 | RPM Package Manager file | rpm, src.rpm |  |
| fmt/795 | RPM Package Manager file | rpm, src.rpm |  |
| linguist/314 | RPM Spec | .spec | text/x-rpm-spec |
| apache-httpd/5789844385051418491 | rsd xml | rsd | application/rsd+xml |
| apache-httpd/12753032465493873976 | rss xml | rss | application/rss+xml |
| linguist/326 | Ruby | .rb, .builder, .eye, .fcgi, .gemspec, .god, .jbuilder, .mspec, .pluginspec, .podspec, .prawn, .rabl, .rake, .rbi, .rbuild, .rbw, .rbx, .ru, .ruby, .spec, .thor, .watchr | text/x-ruby |
| linguist/315 | RUNOFF | .rnh, .rno |  |
| linguist/327 | Rust | .rs, .rs.in | text/x-rustsrc |
| linguist/307 | R | .r, .rd, .rsx | text/x-rsrc |
| fmt/1935 | S-57 Electronic Navigational Chart | 000, 001, 002, 003, 004, 006 |  |
| apache-httpd/14460022341763891094 | s3m | s3m | audio/s3m |
| fmt/887 | SafeGuard Encrypted Virtual Disk | vol, hdr |  |
| linguist/338 | Sage | .sage, .sagews | text/x-python |
| apache-httpd/6902652129543922508 | sailingtracker track | st | application/vnd.sailingtracker.track |
| linguist/339 | SaltStack | .sls | text/x-yaml |
| fmt/1560 | Sample Vision Audio File Format | smp |  |
| fmt/1956 | Sandboxels Save File | sbxls |  |
| x-fmt/354 | SAP Document | ali |  |
| x-fmt/355 | SAS Data File | ssd |  |
| x-fmt/192 | SAS for MS-DOS Catalog | sct |  |
| x-fmt/356 | SAS for MS-DOS Database | ssd |  |
| linguist/340 | Sass | .sass | text/x-sass |
| linguist/328 | SAS | .sas | text/x-sas |
| apache-httpd/7633132484611881254 | sbml xml | sbml | application/sbml+xml |
| x-fmt/109 | Scalable Vector Graphics Compressed | svgz | image/svg+xml |
| fmt/413 | Scalable Vector Graphics Tiny | svg |  |
| fmt/91 | Scalable Vector Graphics | svg | image/svg+xml |
| fmt/92 | Scalable Vector Graphics | svg | image/svg+xml |
| linguist/341 | Scala | .scala, .kojo, .sbt, .sc | text/x-scala |
| linguist/342 | Scaml | .scaml |  |
| fmt/213 | ScanIt Document | sid |  |
| x-fmt/357 | Scanstudio 16-Colour Bitmap | adc |  |
| linguist/619814037 | Scenic | .scenic |  |
| x-fmt/99 | Schedule+ Contacts | scd |  |
| linguist/343 | Scheme | .scm, .sch, .sld, .sls, .sps, .ss | text/x-scheme |
| linguist/344 | Scilab | .sci, .sce, .tst |  |
| x-fmt/146 | Scitex Continuous Tone Bitmap | ct, sct |  |
| fmt/717 | Scream Tracker Module | stm |  |
| fmt/718 | Scream Tracker Module | s3m |  |
| fmt/1091 | Scribus Document | sla, scd | application/vnd.scribus |
| fmt/826 | Scriptware Script Format | sw3 |  |
| linguist/329 | SCSS | .scss | text/x-scss |
| apache-httpd/1489417600909014564 | scvp cv request | scq | application/scvp-cv-request |
| apache-httpd/17692487408723072806 | scvp cv response | scs | application/scvp-cv-response |
| apache-httpd/16843356083413255425 | scvp vp request | spq | application/scvp-vp-request |
| apache-httpd/3800111246559995671 | scvp vp response | spp | application/scvp-vp-response |
| apache-httpd/12792425945248664173 | sdp | sdp | application/sdp |
| x-fmt/189 | SDSC Image Tool Run-Length Encoded Bitmap | rle |  |
| x-fmt/188 | SDSC Image Tool Wavefront Raster Image | rla |  |
| x-fmt/209 | SDSC Image Tool X Window Dump Format | xwd |  |
| fmt/1104 | Seattle FilmWorks SFW Image Format | sfw |  |
| fmt/318 | Secure DjVU | djvu, djv | image/vnd.djvu, image/x-djvu |
| linguist/847830017 | sed | .sed |  |
| apache-httpd/3558484330179102172 | seemail | see | application/vnd.seemail |
| fmt/363 | SEG Y Data Exchange Format | segy |  |
| fmt/1558 | SelF-eXtracting LHA/LZH Compressed Files | sfx |  |
| linguist/345 | Self | .self |  |
| linguist/880010326 | SELinux Policy | .te |  |
| apache-httpd/9088264634842123326 | sema | sema | application/vnd.sema |
| apache-httpd/7766891016013321425 | semd | semd | application/vnd.semd |
| apache-httpd/605004890228926996 | semf | semf | application/vnd.semf |
| fmt/1553 | Septentrio Binary Format | sbf |  |
| fmt/1519 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1520 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1521 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1522 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1523 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1524 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1525 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1526 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1527 | Serif DrawPlus Drawing | dpp, dpa, dpx |  |
| fmt/1528 | Serif DrawPlus Drawing | dpp, dpa |  |
| fmt/827 | Serif DrawPlus Drawing | dpp |  |
| fmt/852 | Serif DrawPlus Drawing | dpp |  |
| fmt/853 | Serif DrawPlus Drawing | dpp |  |
| fmt/1529 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1530 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1531 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1532 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1533 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1534 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1535 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1536 | Serif PagePlus Publication | ppp, ppb, ppx |  |
| fmt/1537 | Serif PagePlus Publication | ppp, ppt |  |
| fmt/671 | Serif PagePlus Publication | ppp |  |
| fmt/672 | Serif PagePlus Publication | ppp |  |
| fmt/673 | Serif PagePlus Publication | ppp |  |
| fmt/674 | Serif PagePlus Publication | ppp |  |
| fmt/675 | Serif PagePlus Publication | ppp |  |
| fmt/676 | Serif PagePlus Publication | ppp |  |
| fmt/677 | Serif PagePlus Publication | ppp |  |
| fmt/678 | Serif PagePlus Publication | ppp |  |
| fmt/679 | Serif PagePlus Publication | ppp |  |
| fmt/680 | Serif PagePlus Publication | ppp |  |
| fmt/681 | Serif PagePlus Publication | ppp |  |
| fmt/1517 | Serif PhotoPlus Image | spp |  |
| fmt/1518 | Serif PhotoPlus Image | spp |  |
| apache-httpd/10045809873570187998 | set payment initiation | setpay | application/set-payment-initiation |
| apache-httpd/3148081406512757481 | set registration initiation | setreg | application/set-registration-initiation |
| apache-httpd/13062495241578132449 | setext | etx | text/x-setext |
| apache-httpd/17986581962061889359 | sfv | sfv | text/x-sfv |
| fmt/1901 | SGI Movie File | mv, movie |  |
| apache-httpd/2280517777109703612 | sgi movie | movie | video/x-sgi-movie |
| apache-httpd/10059028400364481609 | sgi | sgi | image/sgi |
| fmt/1618 | SGML/XML Entity File | ent | application/xml-external-parsed-entity |
| fmt/992 | SHA1 File | sha1 |  |
| fmt/991 | SHA256 File | sha256 |  |
| fmt/1797 | SHA512 File | sha512 |  |
| linguist/664257356 | ShaderLab | .shader |  |
| apache-httpd/10173667125709949723 | shana informed formdata | ifm | application/vnd.shana.informed.formdata |
| apache-httpd/6961800999383898337 | shana informed formtemplate | itp | application/vnd.shana.informed.formtemplate |
| apache-httpd/14364337678005556784 | shana informed interchange | iif | application/vnd.shana.informed.interchange |
| apache-httpd/10655404354733431138 | shana informed package | ipk | application/vnd.shana.informed.package |
| fmt/329 | Shell Archive Format | shar | application/x-sh, application/x-shar |
| linguist/687511714 | ShellCheck Config |  | text/x-properties |
| linguist/347 | ShellSession | .sh-session | text/x-sh |
| linguist/346 | Shell | .sh, .bash, .bats, .cgi, .command, .fcgi, .ksh, .sh.in, .tmux, .tool, .trigger, .zsh, .zsh-theme | text/x-sh |
| linguist/348 | Shen | .shen |  |
| apache-httpd/12821240876237287409 | shf xml | shf | application/shf+xml |
| fmt/1961 | Shorten (codec) | shn |  |
| apache-httpd/15414754435807971579 | sh | sh | application/x-sh |
| fmt/1196 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| fmt/161 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| fmt/1777 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| fmt/995 | SIARD (Software-Independent Archiving of Relational Databases) | siard |  |
| fmt/1994 | Sibelius Scorch | sco |  |
| fmt/1978 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1979 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1980 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1981 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1982 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1983 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1984 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1985 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1986 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1987 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1988 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1989 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1990 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1991 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1992 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1993 | Sibelius Score | sib | application/x-sibelius-score |
| fmt/1229 | Sibelius Sound Set Definition | set |  |
| fmt/696 | Sibelius | sib |  |
| fmt/1148 | SIDOUN WinAVA Format | swa |  |
| fmt/883 | Siegfried Signature File | sig |  |
| linguist/208976687 | Sieve | .sieve | application/sieve |
| fmt/661 | Sigma RAW Image | x3f |  |
| x-fmt/358 | Silicon Graphics Graphics File |  |  |
| x-fmt/140 | Silicon Graphics Image | bw, rgb | image/x-sgi-bw |
| x-fmt/186 | Silicon Graphics RGB File |  |  |
| apache-httpd/17376429027797396733 | silk | sil | audio/silk |
| fmt/1067 | Silo | silo |  |
| fmt/1068 | Silo | silo |  |
| apache-httpd/6212653248367935630 | silverlight app | xap | application/x-silverlight-app |
| linguist/735623761 | Simple File Verification | .sfv | text/x-properties |
| fmt/933 | Simple Vector Format | svf | image/vnd-svf |
| fmt/934 | Simple Vector Format | svf | image/vnd-svf |
| apache-httpd/10132205750207181206 | simtech mindmapper | twd, twds | application/vnd.simtech-mindmapper |
| linguist/987024632 | Singularity |  |  |
| fmt/1230 | SK-XML | ddoc |  |
| fmt/1259 | SketchUp Document | skb, skp |  |
| fmt/1260 | SketchUp Document | skp, skb |  |
| fmt/1261 | SketchUp Document | skp, skb |  |
| fmt/1262 | SketchUp Document | skp, skb |  |
| fmt/1263 | SketchUp Document | skp, skb |  |
| fmt/1264 | SketchUp Document | skp, skb |  |
| fmt/1265 | SketchUp Document | skp, skb |  |
| fmt/1266 | SketchUp Document | skp, skb |  |
| fmt/1267 | SketchUp Document | skp, skb |  |
| fmt/1268 | SketchUp Document | skp, skb |  |
| fmt/1269 | SketchUp Document | skp, skb |  |
| fmt/1270 | SketchUp Document | skp, skb |  |
| fmt/1271 | SketchUp Document | skp, skb |  |
| fmt/1272 | SketchUp Document | skp, skb |  |
| fmt/1273 | SketchUp Document | skp, skb |  |
| x-fmt/451 | SketchUp Document | skb, skp |  |
| x-fmt/452 | SketchUp Document |  |  |
| linguist/349 | Slash | .sl |  |
| linguist/894641667 | Slice | .ice |  |
| linguist/350 | Slim | .slim | text/x-slim |
| linguist/119900149 | Slint | .slint |  |
| fmt/1234 | Smacker Video | smk | video/vnd.radgamettools.smacker |
| apache-httpd/5348486227401194865 | smaf | mmf | application/vnd.smaf |
| linguist/351 | Smali | .smali |  |
| linguist/352 | Smalltalk | .st, .cs | text/x-stsrc |
| apache-httpd/17112233685912069940 | smart teacher | teacher | application/vnd.smart.teacher |
| fmt/623 | SmartDraw | sdr |  |
| linguist/353 | Smarty | .tpl | text/x-smarty |
| apache-httpd/5137819976307855937 | smil xml | smi, smil | application/smil+xml |
| linguist/1027892786 | Smithy | .smithy | text/x-csrc |
| linguist/164123055 | SmPL | .cocci |  |
| linguist/330 | SMT | .smt2, .smt |  |
| apache-httpd/16034155539244253487 | smv | smv | video/x-smv |
| linguist/151241392 | Snakemake | .smk, .snakefile | text/x-python |
| fmt/1057 | SNAP Archive Data File | adf |  |
| fmt/1056 | SNAP Main Data File | mdf |  |
| fmt/1058 | SNAP Processed Data File | snpdf |  |
| fmt/781 | Snoop Packet Capture | snoop |  |
| fmt/1359 | Softdisk Text Compressor | ctx |  |
| fmt/1167 | Softimage 3D Picture File Format | pic |  |
| fmt/1711 | Software602 Printer Configuration File | cfg |  |
| apache-httpd/14282171209386449834 | solent sdkm xml | sdkm, sdkd | application/vnd.solent.sdkm+xml |
| linguist/237469032 | Solidity | .sol |  |
| fmt/1967 | Solidworks Design Document Files | sldprt, slddrw, sldasm, sld, sldlfp, slddrt |  |
| fmt/1962 | SolidWorks Material Database File | sldmat |  |
| fmt/951 | Sonic Foundry WAVE 64 | w64, wav |  |
| fmt/1274 | Sonic Scenarist Closed Caption Format | scc |  |
| fmt/1127 | Sony ARW RAW Image File | arw |  |
| fmt/191 | Sony ARW RAW Image File | arw |  |
| fmt/472 | Sony Digital Voice File/Sony Memory Stick Voice File | msv, dvf |  |
| fmt/1335 | Sony PictureGear Studio Binder | bxu, bxt |  |
| fmt/1333 | Sony PictureGear Studio PhotoAlbum | amu, amd |  |
| fmt/1334 | Sony PictureGear Studio PrintStudio | lmu, lmd |  |
| fmt/1207 | Sony SFK File | sfk |  |
| fmt/1764 | Sony SLV File | slv |  |
| fmt/1766 | Sony SML File | sml |  |
| fmt/1126 | Sony SR2 RAW Image File | sr2 |  |
| linguist/222900098 | Soong |  |  |
| fmt/1246 | SOSI | sos | text/vnd.sosi |
| fmt/1247 | SOSI | sos | text/vnd.sosi |
| fmt/1248 | SOSI | sos | text/vnd.sosi |
| fmt/1249 | SOSI | sos | text/vnd.sosi |
| fmt/1250 | SOSI | sos | text/vnd.sosi |
| fmt/209 | Sound Designer II Audio File | sd2 |  |
| linguist/354 | SourcePawn | .sp, .inc |  |
| fmt/1226 | Sparky | ucsf |  |
| apache-httpd/11948227751138753910 | sparql query | rq | application/sparql-query |
| apache-httpd/3123908653018764713 | sparql results xml | srx | application/sparql-results+xml |
| linguist/331 | SPARQL | .sparql, .rq | application/sparql-query |
| fmt/1575 | Spectrum 512 Compressed | Spectrum 512 Smooshed | spc, sps |  |
| fmt/1577 | Spectrum 512 Extended | spx |  |
| fmt/1578 | Spectrum 512 Extended | spx |  |
| fmt/1576 | Spectrum 512 Uncompressed | Spectrum 512 Uncompressed Enhanced | spu |  |
| x-fmt/132 | Speller Custom Dictionary | dic |  |
| x-fmt/133 | Speller Exclude Dictionary | dic |  |
| fmt/1996 | SPIR-V | spirv |  |
| linguist/767169629 | Spline Font Database | .sfd |  |
| apache-httpd/15134871687334348672 | spotfire dxp | dxp | application/vnd.spotfire.dxp |
| apache-httpd/9182902311269733745 | spotfire sfs | sfs | application/vnd.spotfire.sfs |
| fmt/1561 | SpritePad Image Format | spd |  |
| fmt/638 | SPSS Data File | sav |  |
| fmt/274 | SPSS Output File (spv) | spv |  |
| fmt/1869 | SPSS PC File Format |  |  |
| fmt/997 | SPSS Portable Data Format | por |  |
| fmt/1579 | SPYne Containers | spy |  |
| linguist/332 | SQF | .sqf, .hqf |  |
| fmt/1135 | SQLite Database File Format | sqlite, db |  |
| fmt/729 | SQLite Database File Format | sqlite, db, db3, sqlite3 | application/x-sqlite3 |
| linguist/334 | SQLPL | .sql, .db2 | text/x-sql |
| apache-httpd/5696044814271161966 | sql | sql | application/x-sql |
| linguist/333 | SQL | .sql, .cql, .ddl, .inc, .mysql, .prc, .tab, .udf, .viw | text/x-sql |
| linguist/355 | Squirrel | .nut | text/x-c++src |
| linguist/335 | SRecode Template | .srt | text/x-common-lisp |
| apache-httpd/593596079114199361 | srgs xml | grxml | application/srgs+xml |
| apache-httpd/10097865076610039677 | srgs | gram | application/srgs |
| apache-httpd/17814697578080827582 | sru xml | sru | application/sru+xml |
| apache-httpd/8296049173261629556 | ssdl xml | ssdl | application/ssdl+xml |
| linguist/554920715 | SSH Config |  |  |
| apache-httpd/18215989259559031662 | ssml xml | ssml | application/ssml+xml |
| fmt/1653 | STAD PAC File | pac, seq |  |
| fmt/1555 | Standard Data Format | sdf |  |
| fmt/504 | Standard Flowgram Format | sff |  |
| fmt/698 | Standard for the Exchange of Product model data | step, stp, p21 |  |
| x-fmt/195 | Standard Generalized Markup Language | sgml, sgm | text/sgml |
| linguist/357 | Standard ML | .ml, .fun, .sig, .sml | text/x-ocaml |
| linguist/356 | Stan | .stan |  |
| apache-httpd/13156498984566289220 | stardivision calc | sdc | application/vnd.stardivision.calc |
| apache-httpd/17575372223664686893 | stardivision impress | sdd | application/vnd.stardivision.impress |
| apache-httpd/17689098337773368263 | stardivision math | smf | application/vnd.stardivision.math |
| apache-httpd/2615400360932218329 | stardivision writer global | sgl | application/vnd.stardivision.writer-global |
| linguist/960266174 | Starlark | .bzl, .star | text/x-python |
| fmt/1556 | Starlink Data Format | sdf |  |
| fmt/808 | StarOffice Calc | sdc |  |
| fmt/809 | StarOffice Calc | sdc |  |
| x-fmt/359 | StarOffice Calc | sdc |  |
| x-fmt/404 | StarOffice Calc |  |  |
| fmt/810 | StarOffice Draw | sdd |  |
| fmt/811 | StarOffice Draw | sdd |  |
| x-fmt/401 | StarOffice Draw | sda | application/vnd.stardivision.draw |
| x-fmt/402 | StarOffice Draw |  |  |
| fmt/814 | StarOffice Impress | sdd |  |
| fmt/815 | StarOffice Impress | sdd |  |
| x-fmt/360 | StarOffice Impress | sdd |  |
| x-fmt/405 | StarOffice Impress |  |  |
| fmt/812 | StarOffice Writer | sdw |  |
| fmt/813 | StarOffice Writer | sdw |  |
| x-fmt/400 | StarOffice Writer | sdw | application/vnd.stardivision.writer |
| x-fmt/403 | StarOffice Writer |  |  |
| linguist/424510560 | STAR | .star |  |
| fmt/1598 | Stata .do Command File | do |  |
| fmt/1029 | Stata Data (DTA) Format | dta |  |
| fmt/1030 | Stata Data (DTA) Format | dta |  |
| fmt/1031 | Stata Data (DTA) Format | dta |  |
| fmt/1032 | Stata Data (DTA) Format | dta |  |
| fmt/1033 | Stata Data (DTA) Format | dta |  |
| fmt/1034 | Stata Data (DTA) Format | dta |  |
| fmt/1035 | Stata Data (DTA) Format | dta |  |
| fmt/1036 | Stata Data (DTA) Format | dta |  |
| fmt/1037 | Stata Data (DTA) Format | dta |  |
| linguist/358 | Stata | .do, .ado, .doh, .ihlp, .mata, .matah, .sthlp |  |
| x-fmt/361 | StatGraphics Data File | aws |  |
| x-fmt/131 | Stationery for Mac OS X | doc |  |
| fmt/210 | Statistica Report File | str |  |
| fmt/1024 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| fmt/1026 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| fmt/1028 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| fmt/606 | Statistical Analysis System Catalog (Unix) | sas7bcat, sc7 |  |
| fmt/1023 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| fmt/1025 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| fmt/1027 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| fmt/605 | Statistical Analysis System Catalog (Windows) | sas7bcat, sc7 |  |
| fmt/602 | Statistical Analysis System Catalogue XPT (Unix) | xpt |  |
| fmt/601 | Statistical Analysis System Catalogue XPT (Windows) | xpt |  |
| fmt/1016 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| fmt/1018 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| fmt/1020 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| fmt/1022 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| fmt/608 | Statistical Analysis System Data (Unix) | sas7bdat, sd7 |  |
| fmt/1015 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| fmt/1017 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| fmt/1019 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| fmt/1021 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| fmt/607 | Statistical Analysis System Data (Windows) | sas7bdat, sd7 |  |
| fmt/604 | Statistical Analysis System Data XPT (Unix) | xpt |  |
| fmt/603 | Statistical Analysis System Data XPT (Windows) | xpt |  |
| x-fmt/145 | Stats+ Data File |  |  |
| x-fmt/449 | Steel Detailing Neutral Format | sdn | text/plain |
| apache-httpd/6070408414635237449 | stepmania package | smzip | application/vnd.stepmania.package |
| apache-httpd/12775287888660207002 | stepmania stepchart | sm | application/vnd.stepmania.stepchart |
| fmt/112 | Still Picture Interchange File Format | spf, jpg | image/jpeg |
| fmt/113 | Still Picture Interchange File Format |  | image/jpeg |
| x-fmt/108 | STL (Standard Tessellation Language) ASCII | stl |  |
| fmt/865 | STL (Standard Tessellation Language) Binary | stl |  |
| linguist/455361735 | STL | .stl |  |
| linguist/336 | STON | .ston |  |
| fmt/1467 | STOS Memory Bank | mbk |  |
| fmt/1204 | Strata StudioPro Vis Format |  |  |
| x-fmt/362 | StratGraphics Data File | asf |  |
| linguist/89855901 | StringTemplate | .st | text/html |
| fmt/245 | Structured Data Exchange Format |  |  |
| fmt/206 | Structured Query Language Data | sql |  |
| fmt/1402 | Student Writing Center Journal | jn, jnt |  |
| fmt/1405 | Student Writing Center Letter | lt, ltt |  |
| fmt/1404 | Student Writing Center Newsletter | nl, nlt |  |
| fmt/1401 | Student Writing Center Report | rp, rpt |  |
| fmt/1403 | Student Writing Center Sign | sg, sgt |  |
| fmt/1459 | Stuffit Archive File | sit | application/x-stuffit |
| fmt/1460 | Stuffit Archive File | sit | application/x-stuffit |
| fmt/639 | Stuffit Archive File | sit | application/x-stuffit |
| fmt/399 | Stuffit X Archive File | sitx |  |
| apache-httpd/1762984859953048466 | stuffitx | sitx | application/x-stuffitx |
| linguist/359 | Stylus | .styl |  |
| fmt/1218 | SubRip Subtitle File | srt |  |
| linguist/360 | SubRip Text | .srt |  |
| apache-httpd/12175115353246933006 | subrip | srt | application/x-subrip |
| linguist/826404698 | SugarSS | .sss |  |
| apache-httpd/11118749754585272774 | sun j2me app descriptor | jad | text/vnd.sun.j2me.app-descriptor |
| x-fmt/184 | Sun Raster Image | ras, sun | image/x-sun-raster |
| apache-httpd/17986501611929889311 | sun xml calc template | stc | application/vnd.sun.xml.calc.template |
| apache-httpd/6782189470640064892 | sun xml draw template | std | application/vnd.sun.xml.draw.template |
| apache-httpd/9051819284956207821 | sun xml impress template | sti | application/vnd.sun.xml.impress.template |
| apache-httpd/13582896711578522359 | sun xml math | sxm | application/vnd.sun.xml.math |
| apache-httpd/13923193796103206743 | sun xml writer global | sxg | application/vnd.sun.xml.writer.global |
| apache-httpd/8267445119885387718 | sun xml writer template | stw | application/vnd.sun.xml.writer.template |
| fmt/403 | SuperCalc Spreadsheet | cal |  |
| x-fmt/363 | SuperCalc Spreadsheet | cal |  |
| x-fmt/364 | SuperCalc Spreadsheet | cal |  |
| linguist/361 | SuperCollider | .sc, .scd |  |
| fmt/734 | SuperScape Virtual Reality Format | svr |  |
| fmt/1276 | SureThing Project File | std |  |
| fmt/1552 | Surprise! Adlib Tracker v2.0 | sa2 |  |
| apache-httpd/10548945477827165241 | sus calendar | sus, susp | application/vnd.sus-calendar |
| apache-httpd/7226557976762131676 | sv4cpio | sv4cpio | application/x-sv4cpio |
| apache-httpd/17467597544118241061 | sv4crc | sv4crc | application/x-sv4crc |
| apache-httpd/7128109369329973531 | svd | svd | application/vnd.svd |
| linguist/928734530 | Svelte | .svelte | text/html |
| linguist/337 | SVG | .svg | text/xml |
| linguist/271471144 | Sway | .sw | text/x-rustsrc |
| linguist/558779190 | Sweave | .rnw |  |
| linguist/362 | Swift | .swift | text/x-swift |
| linguist/1066250075 | SWIG | .i | text/x-c++src |
| fmt/1865 | SWiSH Movie File | swi |  |
| fmt/1583 | SXG (ZX Spectrum) Graphic File | sxg |  |
| apache-httpd/16923033345032457 | symbian install | sis, sisx | application/vnd.symbian.install |
| fmt/205 | Synchronized Multimedia Integration Language (Generic) | smil, smi |  |
| apache-httpd/11342459016954218595 | syncml dm wbxml | bdm | application/vnd.syncml.dm+wbxml |
| apache-httpd/18350820160319142221 | syncml dm xml | xdm | application/vnd.syncml.dm+xml |
| apache-httpd/16688631424076391908 | syncml xml | xsm | application/vnd.syncml+xml |
| fmt/1178 | Synthetic Music Mobile Application Format | mmf | application/vnd.yamaha.smaf-audio |
| linguist/363 | SystemVerilog | .sv, .svh, .vh | text/x-systemverilog |
| apache-httpd/15083548841589849073 | t3vm image | t3 | application/x-t3vm-image |
| fmt/820 | T64 Tape Image Format | t64 |  |
| x-fmt/13 | Tab-separated Values | tsv, tab | text/tab-separated-values |
| linguist/606708469 | Tact | .tact |  |
| apache-httpd/2705387182677033387 | tads | gam | application/x-tads |
| fmt/154 | Tagged Image File Format for Electronic Photography (TIFF/EP) | tif, tiff | image/tiff |
| fmt/153 | Tagged Image File Format for Image Technology (TIFF/IT) | tif, tiff | image/tiff |
| fmt/156 | Tagged Image File Format for Internet Fax (TIFF-FX) | tif, tiff, tfx | image/tiff |
| fmt/10 | Tagged Image File Format |  |  |
| fmt/353 | Tagged Image File Format | tif, tiff | image/tiff |
| fmt/7 | Tagged Image File Format |  |  |
| fmt/8 | Tagged Image File Format |  |  |
| fmt/9 | Tagged Image File Format |  |  |
| linguist/959889508 | Talon | .talon |  |
| apache-httpd/9814302341999196290 | tao intent module archive | tao | application/vnd.tao.intent-module-archive |
| fmt/802 | TAP (Commodore 64) | tap |  |
| fmt/801 | TAP (ZX Spectrum) | tap |  |
| x-fmt/265 | Tape Archive Format | tar | application/x-tar |
| fmt/1589 | Taquart Interlace Picture | tip |  |
| apache-httpd/15812306591423857328 | tcl | tcl | application/x-tcl |
| linguist/367 | Tcl | .tcl, .adp, .sdc, .tcl.in, .tm, .xdc | text/x-tcl |
| fmt/1099 | TCR eBook | tcr |  |
| linguist/368 | Tcsh | .tcsh, .csh | text/x-sh |
| linguist/370 | Tea | .tea |  |
| fmt/1475 | TEI P4 XML - Corpus File | xml, tei, odd | application/tei+xml |
| fmt/1474 | TEI P4 XML - Single Text File | xml, tei, odd | application/tei+xml |
| fmt/1476 | TEI P5 - Single Text File | xml, tei, odd | application/tei+xml |
| fmt/1477 | TEI P5 XML - Corpus File | xml, tei, odd | application/tei+xml |
| linguist/795579337 | templ | .templ |  |
| linguist/856832701 | Terraform Template | .tftpl | text/x-ruby |
| linguist/371 | Terra | .t | text/x-lua |
| x-fmt/365 | TeX Binary File | dvi | application/x-dvi |
| apache-httpd/17620094873161353617 | tex tfm | tfm | application/x-tex-tfm |
| fmt/160 | TeX/LaTeX Device Independent Document | dvi | application/x-dvi |
| apache-httpd/11775578050658360080 | texinfo | texinfo, texi | application/x-texinfo |
| linguist/988020015 | Texinfo | .texinfo, .texi, .txi |  |
| x-fmt/421 | Text Configuration file | ini |  |
| linguist/965696054 | TextGrid | .TextGrid |  |
| linguist/373 | Textile | .textile | text/x-textile |
| linguist/981795023 | TextMate Properties |  | text/x-properties |
| default/1 | Text |  | text/plain |
| linguist/372 | Text | .txt, .fr, .nb, .ncl, .no |  |
| apache-httpd/3876443208898332249 | tex | tex | application/x-tex |
| linguist/369 | TeX | .tex, .aux, .bbx, .cbx, .cls, .dtx, .ins, .lbx, .ltx, .mkii, .mkiv, .mkvi, .sty, .toc | text/x-stex |
| apache-httpd/6911424546062404041 | tga | tga | image/x-tga |
| fmt/1588 | TGIF File Format | tgif, obj |  |
| apache-httpd/4451768154165309081 | tgif | obj | application/x-tgif |
| fmt/1094 | The Neuroimaging Informatics Technology Initiative File Format | nii |  |
| fmt/798 | The Neuroimaging Informatics Technology Initiative File Format | nii |  |
| fmt/1301 | The Print Shop Project | psproj |  |
| fmt/1782 | The Spectral Geologist Dataset | tsg |  |
| fmt/1783 | The Spectral Geologist Dataset | tsg |  |
| fmt/1586 | TheDraw Save File | td |  |
| apache-httpd/16755885106588162713 | thraud xml | tfi | application/thraud+xml |
| linguist/374 | Thrift | .thrift |  |
| fmt/682 | Thumbs DB file | db | application/vnd.microsoft.windows.thumbnail-cache |
| linguist/422 | TI Program | .8xp, .8xp.txt |  |
| fmt/1909 | TibetDoc Word Document | dct |  |
| fmt/1717 | Time Stamp Token | tst | application/vnd.etsi.timestamp-token |
| fmt/1487 | Timeline Maker Document | tlm, tlm3, tlm4, tlmp |  |
| apache-httpd/1889659990446513902 | timestamped data | tsd | application/timestamped-data |
| linguist/118656070 | TL-Verilog | .tlv |  |
| linguist/364 | TLA | .tla |  |
| apache-httpd/946043484823141794 | tmobile livetv | tmo | application/vnd.tmobile-livetv |
| linguist/356554395 | Toit | .toit |  |
| linguist/365 | TOML | .toml | text/x-toml |
| fmt/1802 | Transcriber AG TAG Format | tag |  |
| fmt/1803 | Transcriber TRS Format | trs |  |
| fmt/496 | TransXchange File Format | txc |  |
| fmt/1848 | Trelby Document File | trelby |  |
| apache-httpd/7943339296956635151 | trid tpt | tpt | application/vnd.trid.tpt |
| fmt/1085 | TRIM Context Reference File | tr5, txt |  |
| apache-httpd/11556186021095517549 | triscape mxs | mxs | application/vnd.triscape.mxs |
| apache-httpd/15474808736122015508 | troff | t, tr, roff, man, me, ms | text/troff |
| fmt/952 | True Audio | tta | audio/tta |
| fmt/953 | True Audio | tta | audio/tta |
| fmt/1607 | True Colour Picture [Spooky Sprites] (Atari Falcon) | trp, tru |  |
| fmt/1605 | True Colour Sprites [Spooky Sprites] (Atari Falcon) | trs |  |
| apache-httpd/918813325830249825 | trueapp | tra | application/vnd.trueapp |
| x-fmt/453 | TrueType Font | ttf | font/ttf |
| fmt/402 | Truevision TGA Bitmap | tga, icb, vda, vst |  |
| x-fmt/367 | Truevision TGA Bitmap | tga, icb, vda, vst, afi, bpx |  |
| linguist/89289301 | TSPLIB data | .tsp |  |
| linguist/918334941 | TSQL | .sql |  |
| linguist/1035892117 | TSV | .tsv, .vcf |  |
| linguist/94901924 | TSX | .tsx | text/jsx |
| fmt/1603 | TUNDRA | tnd |  |
| x-fmt/199 | Turbo Debugger Keystroke Recording File | tdk |  |
| fmt/1585 | TurboCalc Document | tcd |  |
| linguist/375 | Turing | .t, .tu |  |
| fmt/874 | Turtle | ttl | text/turtle |
| linguist/376 | Turtle | .ttl | text/turtle |
| fmt/1311 | Tweet JSON | json | application/json |
| linguist/377 | Twig | .twig | text/x-twig |
| linguist/366 | TXL | .txl |  |
| linguist/632765617 | Type Language | .tl |  |
| fmt/1601 | Type Library | tlb |  |
| fmt/1602 | Type Library | tlb |  |
| fmt/1652 | Typescript | ts, tsx |  |
| linguist/378 | TypeScript | .ts, .cts, .mts | application/typescript |
| linguist/952272597 | TypeSpec | .tsp |  |
| linguist/704730682 | Typst | .typ |  |
| fmt/1000 | TZX Format | tzx |  |
| fmt/1738 | UDF Disc Image | toast, iso, cdr, dmg |  |
| fmt/1739 | UDF-ISO 9660 Bridge Disc | toast, iso, cdr, dmg |  |
| apache-httpd/18233141180189460471 | ufdl | ufd, ufdl | application/vnd.ufdl |
| apache-httpd/13562036474254073458 | uiq theme | utz | application/vnd.uiq.theme |
| apache-httpd/5595356910571652011 | umajin | umj | application/vnd.umajin |
| x-fmt/16 | Unicode Text File |  | text/plain |
| fmt/792 | Unified Emulator Format | uef, hq.uef |  |
| linguist/379 | Unified Parallel C | .upc | text/x-csrc |
| fmt/1478 | Unisig |  |  |
| x-fmt/193 | Unisys (Sperry) System Data File | sdf |  |
| linguist/380 | Unity3D Asset | .anim, .asset, .mask, .mat, .meta, .prefab, .unity | text/x-yaml |
| apache-httpd/18397815986228800869 | unity | unityweb | application/vnd.unity |
| fmt/702 | Universal 3D File Format | u3d |  |
| fmt/1905 | Universal Scene Description ASCII File | usda |  |
| linguist/120 | Unix Assembly | .s, .ms |  |
| linguist/381 | Uno | .uno | text/x-csharp |
| linguist/382 | UnrealScript | .uc | text/x-java |
| apache-httpd/8338995056510031503 | uoml xml | uoml | application/vnd.uoml+xml |
| apache-httpd/14463557769074081415 | uri list | uri, uris, urls | text/uri-list |
| linguist/383 | UrWeb | .ur, .urs |  |
| apache-httpd/347415101871463480 | ustar | ustar | application/x-ustar |
| fmt/1102 | Uuencoded File | uue |  |
| apache-httpd/13440853329793760916 | uuencode | uu | text/x-uuencode |
| apache-httpd/13221059779387609093 | uvvu mp4 | uvu, uvvu | video/vnd.uvvu.mp4 |
| fmt/1364 | V-Ray Material | vismat |  |
| linguist/386 | Vala | .vala, .vapi |  |
| linguist/544060961 | Valve Data Format | .vdf |  |
| fmt/985 | Valve Texture Format | vtf | image/vnd.valve.source.texture |
| fmt/1122 | VAMAS Surface Chemical Analysis Standard Data Transfer Format | vms |  |
| fmt/905 | Variant Call Format | vcf |  |
| fmt/906 | Variant Call Format | vcf |  |
| fmt/907 | Variant Call Format | vcf |  |
| fmt/908 | Variant Call Format | vcf |  |
| fmt/1381 | VariCAD Drawing | dwb |  |
| linguist/399230729 | VBA | .bas, .cls, .frm, .vba | text/x-vb |
| fmt/1906 | VBM (VDC BitMap) File | vbm |  |
| fmt/1089 | VBScript (VBS) File | vbs |  |
| linguist/408016005 | VBScript | .vbs | text/vbscript |
| fmt/387 | VCalendar format | vcs | text/x-vCalendar |
| apache-httpd/3360421230686974468 | vcalendar | vcs | text/x-vcalendar |
| apache-httpd/2856549286746833970 | vcard | vcf | text/x-vcard |
| fmt/1879 | vCard | vcf, vcard | text/vcard |
| fmt/1880 | vCard | vcf, vcard | text/vcard |
| fmt/1881 | vCard | vcf, vcard | text/vcard |
| fmt/395 | vCard | vcf, vcard | text/vcard |
| linguist/851476558 | vCard | .vcf | text/x-properties |
| linguist/384 | VCL | .vcl |  |
| apache-httpd/16181652763127625250 | vcx | vcx | application/vnd.vcx |
| fmt/583 | Vector Markup Language | vml, html, htm |  |
| fmt/1142 | VectorWorks Plugin or Script | vso, vst, vsm | application/vnd.vectorworks |
| fmt/1139 | VectorWorks | vwx | application/vnd.vectorworks |
| fmt/1140 | VectorWorks | vwx | application/vnd.vectorworks |
| fmt/1141 | VectorWorks | vwx | application/vnd.vectorworks |
| fmt/450 | VectorWorks | vwx | application/vnd.vectorworks |
| fmt/451 | VectorWorks | vwx | application/vnd.vectorworks |
| fmt/684 | Vectorworks | vwx | application/vnd.vectorworks |
| fmt/686 | Vectorworks | vwx | application/vnd.vectorworks |
| linguist/292377326 | Velocity Template Language | .vtl | text/velocity |
| x-fmt/57 | Ventura Publisher Vector Graphics | gem |  |
| x-fmt/156 | Ventura Publisher | gen |  |
| linguist/387 | Verilog | .v, .veo | text/x-verilog |
| fmt/457 | Verity Collection Document Dataset Descriptor Style Set | ddd |  |
| fmt/458 | Verity Collection Document Index Descriptor Style Set | did |  |
| fmt/454 | Verity Collection Index About File | abt |  |
| fmt/461 | Verity Collection Index Descriptor File | wld, ddd, did, pdd |  |
| fmt/455 | Verity Collection Index Pending Transaction File | trn |  |
| fmt/456 | Verity Collection Index Style Policy | plc |  |
| fmt/460 | Verity Collection Partition Definition Descriptor Style Set | pdd |  |
| fmt/453 | Verity Collection Stop List | stp |  |
| fmt/459 | Verity Collection Word List Descriptor Style Set | wld |  |
| linguist/385 | VHDL | .vhdl, .vhd, .vhf, .vhi, .vho, .vhs, .vht, .vhw | text/x-vhdl |
| fmt/1610 | Viacom New Media Graphics | vnm, 000 |  |
| fmt/383 | VICAR (Video Image Communication and Retrieval) Planetary File Format | img, vic, vicar |  |
| fmt/425 | Video Object File (MPEG-2 subset) | vob |  |
| linguist/508563686 | Vim Help File | .txt |  |
| linguist/388 | Vim Script | .vim, .vba, .vimrc, .vmb |  |
| linguist/81265970 | Vim Snippet | .snip, .snippet, .snippets |  |
| fmt/1582 | Vim SWAP File | swp |  |
| fmt/1811 | Vips Image | v, vips |  |
| fmt/1208 | Virtools File Format | cmo, nmo, vmo, nms |  |
| fmt/726 | Virtual Disk Image | vdi |  |
| fmt/1356 | Virtual Format (Raster) | vrt |  |
| fmt/1357 | Virtual Format (Vector) | vrt |  |
| fmt/93 | Virtual Reality Modeling Language | wrl | model/vrml |
| fmt/94 | Virtual Reality Modeling Language | wrl | model/vrml |
| x-fmt/368 | VisiCalc Database | dif |  |
| apache-httpd/4738532386085573748 | visionary | vis | application/vnd.visionary |
| x-fmt/369 | Vista Pro Graphics | dem |  |
| fmt/1088 | Visual Basic (VB) File | vb |  |
| linguist/389 | Visual Basic .NET | .vb, .vbhtml | text/x-vb |
| linguist/679594952 | Visual Basic 6.0 | .bas, .cls, .ctl, .Dsr, .frm | text/x-vb |
| fmt/1590 | Visual Basic Binary Form File | frx |  |
| fmt/1541 | Visual Basic Form File | frm |  |
| fmt/1542 | Visual Basic Form File | frm |  |
| x-fmt/48 | Visual Basic Macro | dvb |  |
| fmt/1573 | Visual Basic Project File | vbp |  |
| fmt/1574 | Visual Basic Project Workspace File | vbw |  |
| fmt/1548 | Visual Basics MAK File | mak |  |
| x-fmt/150 | Visual FoxPro Database Container File | dcx |  |
| fmt/499 | VivoActive | viv | video/vnd-vivo |
| apache-httpd/1757511704341130711 | vivo | viv | video/vnd.vivo |
| fmt/721 | VLW Font File | vlw |  |
| apache-httpd/7319119812613679345 | voicexml xml | vxml | application/voicexml+xml |
| linguist/390 | Volt | .volt | text/x-d |
| apache-httpd/17932304242732550516 | vsf | vsf | application/vnd.vsf |
| apache-httpd/9317680701060824198 | vtu | vtu | model/vnd.vtu |
| linguist/391 | Vue | .vue |  |
| linguist/1055641948 | Vyper | .vy |  |
| linguist/603371597 | V | .v | text/x-go |
| fmt/1840 | WACZ | wacz | application/x-wacz |
| apache-httpd/12506654617699971632 | wais source | src | application/x-wais-source |
| apache-httpd/16355469457160075364 | wap wbmp | wbmp | image/vnd.wap.wbmp |
| apache-httpd/866180193996727481 | wap wbxml | wbxml | application/vnd.wap.wbxml |
| apache-httpd/13947353468228148623 | wap wmlc | wmlc | application/vnd.wap.wmlc |
| apache-httpd/15817187641996057495 | wap wmlscriptc | wmlsc | application/vnd.wap.wmlscriptc |
| apache-httpd/5953949104867887994 | wap wmlscript | wmls | text/vnd.wap.wmlscript |
| fmt/1281 | WARC | warc | application/warc |
| fmt/1355 | WARC | warc | application/warc |
| fmt/289 | WARC | warc | application/warc |
| apache-httpd/4399502372078247101 | wasm | wasm | application/wasm |
| fmt/141 | Waveform Audio (PCMWAVEFORMAT) | wav, wave | audio/x-wav |
| fmt/142 | Waveform Audio (WAVEFORMATEX) | wav, wave | audio/x-wav |
| fmt/143 | Waveform Audio (WAVEFORMATEXTENSIBLE) | wav, wave | audio/x-wav |
| fmt/6 | Waveform Audio | wav | audio/x-wav |
| fmt/1211 | Wavefront Material Template Library | mtl |  |
| linguist/392 | Wavefront Material | .mtl |  |
| fmt/1210 | Wavefront OBJ File | obj |  |
| linguist/393 | Wavefront Object | .obj |  |
| linguist/374521672 | WDL | .wdl |  |
| linguist/394 | Web Ontology Language | .owl |  |
| fmt/1172 | Web Open Font Format | woff2 | font/woff2 |
| fmt/616 | Web Open Font Format | woff | font/woff |
| fmt/1454 | Web Video Text Tracks (WebVTT) Format | vtt | text/vtt |
| linguist/134534086 | WebAssembly Interface Type | .wit | text/x-webidl |
| linguist/956556503 | WebAssembly | .wast, .wat | text/x-common-lisp |
| linguist/395 | WebIDL | .webidl | text/x-webidl |
| apache-httpd/8348285458340242731 | webm | weba | audio/webm |
| fmt/573 | WebM | webm | video/webm |
| fmt/566 | WebP | webp | image/webp |
| fmt/567 | WebP | webp | image/webp |
| fmt/568 | WebP | webp | image/webp |
| apache-httpd/11891847532744855460 | webturbo | wtb | application/vnd.webturbo |
| linguist/658679714 | WebVTT | .vtt |  |
| linguist/668457123 | Wget Config |  |  |
| linguist/836605993 | WGSL | .wgsl |  |
| linguist/888779559 | Whiley | .whiley |  |
| apache-httpd/14032609428049303413 | widget | wgt | application/widget |
| linguist/228 | Wikitext | .mediawiki, .wiki, .wikitext |  |
| linguist/950967261 | Win32 Message File | .mc | text/x-properties |
| fmt/1255 | Windows Address Book | wab |  |
| fmt/114 | Windows Bitmap | ddb, bmp | image/bmp |
| fmt/115 | Windows Bitmap | bmp, dib | image/bmp |
| fmt/116 | Windows Bitmap | dib, bmp | image/bmp |
| fmt/117 | Windows Bitmap | dib, bmp | image/bmp |
| fmt/118 | Windows Bitmap | bmp, dib | image/bmp |
| fmt/119 | Windows Bitmap | bmp, dib | image/bmp |
| x-fmt/414 | Windows Cabinet File | cab | application/vnd.ms-cab-compressed |
| fmt/474 | Windows Help File | hlp |  |
| fmt/614 | Windows Imaging Format | wim, swm |  |
| fmt/1051 | Windows Journal Format | jnt, jtp |  |
| fmt/132 | Windows Media Audio | wma, asf | audio/x-ms-wma |
| fmt/584 | Windows Media Metafile | wmx, wax, wvx, asx |  |
| fmt/589 | Windows Media Playlist | wpl | application/vnd.ms-wpl |
| fmt/441 | Windows Media Video (WVC1) | wmv |  |
| fmt/133 | Windows Media Video | asf, wmv | video/x-ms-wmv |
| x-fmt/119 | Windows Metafile Image | wmf | image/wmf |
| x-fmt/410 | Windows New Executable | exe |  |
| fmt/899 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| fmt/900 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| x-fmt/411 | Windows Portable Executable | exe, dll, sys | application/vnd.microsoft.portable-executable |
| linguist/969674868 | Windows Registry Entries | .reg | text/x-properties |
| x-fmt/420 | Windows Setup File | inf | application/inf |
| fmt/1995 | WinFax Fax Image | fxr, fxm, fxs |  |
| apache-httpd/13710016921932272208 | winhlp | hlp | application/winhlp |
| fmt/497 | Wireless Bitmap | wbmp | image/vnd-wap-wbmp |
| fmt/1796 | Wireless Markup Language (WML) Document | wml | text/vnd.wap.wml |
| linguist/420 | wisp | .wisp | text/x-clojure |
| linguist/686821385 | Witcher Script | .ws |  |
| apache-httpd/10008482833353115188 | wolfram player | nbp | application/vnd.wolfram.player |
| linguist/632745969 | Wollok | .wlk |  |
| fmt/1723 | Wordcraft Chapter Files | 001 |  |
| fmt/1424 | WordPerfect Encrypted Document | wp | application/vnd.wordperfect |
| fmt/1220 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| fmt/1221 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| fmt/1222 | WordPerfect for Macintosh Document |  | application/vnd.wordperfect |
| x-fmt/393 | WordPerfect for MS-DOS Document | wp, wp5, wpd, w50, doc | application/vnd.wordperfect |
| x-fmt/394 | WordPerfect for MS-DOS/Windows Document | wp5, wpd, w51, wp, doc | application/vnd.wordperfect |
| x-fmt/44 | WordPerfect for MS-DOS/Windows Document | doc, wpd, wp6, wp, w60, w61, w62 | application/vnd.wordperfect |
| x-fmt/203 | WordPerfect for Windows Document | w52, wp, wpd, wp5 | application/vnd.wordperfect |
| fmt/1042 | WordPerfect Graphics Metafile | wpg |  |
| x-fmt/395 | WordPerfect Graphics Metafile | wpg |  |
| fmt/1850 | WordPerfect Macro File | wpm, wcm |  |
| fmt/1837 | WordPerfect Presentations | shw |  |
| x-fmt/42 | Wordperfect Secondary File | doc |  |
| x-fmt/43 | Wordperfect Secondary File | doc |  |
| fmt/949 | WordPerfect | wp4, wpd | application/vnd.wordperfect |
| fmt/882 | Wordstar 2000 |  |  |
| x-fmt/205 | WordStar for MS-DOS Document | ws, ws5 |  |
| x-fmt/236 | WordStar for MS-DOS Document | ws |  |
| x-fmt/237 | WordStar for MS-DOS Document | ws, ws6 |  |
| x-fmt/260 | WordStar for MS-DOS Document | ws, ws4 |  |
| x-fmt/261 | WordStar for MS-DOS Document | ws, ws7 |  |
| x-fmt/370 | WordStar for MS-DOS Document | ws3, ws |  |
| x-fmt/206 | WordStar for Windows Document | wsd, ws, wsw |  |
| x-fmt/262 | WordStar for Windows Document | ws, wsw |  |
| x-fmt/5 | Works for Macintosh Document |  |  |
| linguist/396 | World of Warcraft Addon Data | .toc |  |
| apache-httpd/17504618014497501474 | wqd | wqd | application/vnd.wqd |
| fmt/1611 | WRAptor Compressed File | wra, wr3 |  |
| linguist/713580619 | Wren | .wren |  |
| x-fmt/12 | Write for Windows Document | wri |  |
| x-fmt/4 | Write for Windows Document | wri |  |
| fmt/799 | WriteNow |  |  |
| apache-httpd/2487306939359366350 | wsdl xml | wsdl | application/wsdl+xml |
| apache-httpd/269849346801687012 | wspolicy xml | wspolicy | application/wspolicy+xml |
| apache-httpd/1680085651608889131 | wt stf | stf | application/vnd.wt.stf |
| linguist/782911107 | X BitMap | .xbm | text/x-csrc |
| linguist/208700028 | X Font Directory Index |  |  |
| linguist/781846279 | X PixMap | .xpm, .pm | text/x-csrc |
| x-fmt/207 | X-Windows Bitmap Image | xbm | image/x-xbitmap |
| x-fmt/299 | X-Windows Bitmap Image | xbm | image/x-xbitmap |
| x-fmt/208 | X-Windows Pixmap Image | xpm | image/x-xpixmap |
| x-fmt/300 | X-Windows Screen Dump File | xdm, xwd | image/x-xwindowdump |
| fmt/401 | X-Windows Screen Dump | xwd, xdm |  |
| linguist/397 | X10 | .x10 |  |
| apache-httpd/6898497964742600926 | x3d binary | x3db, x3dbz | model/x3d+binary |
| apache-httpd/13507959258524992275 | x3d vrml | x3dv, x3dvz | model/x3d+vrml |
| apache-httpd/2323420538982214786 | x3d xml | x3d, x3dz | model/x3d+xml |
| fmt/579 | X3D | x3d |  |
| fmt/580 | X3D | x3d |  |
| fmt/581 | X3D | x3d |  |
| fmt/582 | X3D | x3d |  |
| apache-httpd/16814499624578705614 | x509 ca cert | der, crt | application/x-x509-ca-cert |
| fmt/805 | XAML Binary Format | xbf |  |
| apache-httpd/12986059742410892224 | xaml xml | xaml | application/xaml+xml |
| fmt/922 | Xar Image Format | xar |  |
| apache-httpd/9915205638149722777 | xara | xar | application/vnd.xara |
| linguist/421 | xBase | .prg, .ch, .prw |  |
| fmt/1612 | XBIN (eXtended BIN) | xb |  |
| apache-httpd/7452515137701384940 | xcap diff xml | xdf | application/xcap-diff+xml |
| linguist/225167241 | XCompose |  |  |
| linguist/398 | XC | .xc | text/x-csrc |
| fmt/1374 | xdomea | xml |  |
| fmt/1375 | xdomea | xml |  |
| fmt/1376 | xdomea | xml |  |
| fmt/1377 | xdomea | xml |  |
| fmt/1378 | xdomea | xml |  |
| fmt/1379 | xdomea | xml |  |
| fmt/1380 | xdomea |  |  |
| fmt/1813 | xdomea | xml |  |
| apache-httpd/5638618832123976135 | xenc xml | xenc | application/xenc+xml |
| apache-httpd/16561078434078613689 | xfdl | xfdl | application/vnd.xfdl |
| apache-httpd/10075710292268051668 | xfig | fig | application/x-xfig |
| apache-httpd/17748263794182868405 | xhtml xml | xhtml, xht | application/xhtml+xml |
| fmt/1479 | XIFF (Xerox Image File Format) | xif | image/vnd.xiff |
| fmt/1480 | XIFF (Xerox Image File Format) | xif | image/vnd.xiff |
| fmt/1657 | XIMG (Extended GEM Bit Image) | ximg, img |  |
| fmt/1658 | XL-Paint MaX | max, xlp |  |
| fmt/1659 | XL-Paint | raw |  |
| fmt/1447 | XLD4 (Bitmap Image) | q4 |  |
| fmt/1448 | XLD4 (Graphic Data Document) | q4d |  |
| apache-httpd/3716723091890390689 | xliff xml | xlf | application/x-xliff+xml |
| fmt/1828 | XLSX Strict OOXML Spreadsheet | xlsx | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| apache-httpd/3113384014745721199 | xml dtd | dtd | application/xml-dtd |
| fmt/1501 | XML Forms Data Format | xfdf | application/vnd.adobe.xfdf |
| fmt/979 | XML Property List | plist |  |
| linguist/75622871 | XML Property List | .plist, .stTheme, .tmCommand, .tmLanguage, .tmPreferences, .tmSnippet, .tmTheme | text/xml |
| x-fmt/280 | XML Schema Definition | xsd | application/xml |
| fmt/1613 | XML Shareable Playlist Format | xspf |  |
| linguist/399 | XML | .xml, .adml, .admx, .ant, .axaml, .axml, .builds, .ccproj, .ccxml, .clixml, .cproject, .cscfg, .csdef, .csl, .csproj, .ct, .depproj, .dita, .ditamap, .ditaval, .dll.config, .dotsettings, .filters, .fsproj, .fxml, .glade, .gml, .gmx, .grxml, .gst, .hzp, .iml, .ivy, .jelly, .jsproj, .kml, .launch, .mdpolicy, .mjml, .mm, .mod, .mojo, .mxml, .natvis, .ncl, .ndproj, .nproj, .nuspec, .odd, .osm, .pkgproj, .pluginspec, .proj, .props, .ps1xml, .psc1, .pt, .qhelp, .rdf, .res, .resx, .rs, .rss, .sch, .scxml, .sfproj, .shproj, .srdf, .storyboard, .sublime-snippet, .sw, .targets, .tml, .ts, .tsx, .typ, .ui, .urdf, .ux, .vbproj, .vcxproj, .vsixmanifest, .vssettings, .vstemplate, .vxml, .wixproj, .workflow, .wsdl, .wsf, .wxi, .wxl, .wxs, .x3d, .xacro, .xaml, .xib, .xlf, .xliff, .xmi, .xml.dist, .xmp, .xproj, .xsd, .xspec, .xul, .zcml | text/xml |
| linguist/405 | Xojo | .xojo_code, .xojo_menu, .xojo_report, .xojo_script, .xojo_toolbar, .xojo_window |  |
| linguist/614078284 | Xonsh | .xsh | text/x-python |
| apache-httpd/10149397603949644781 | xop xml | xop | application/xop+xml |
| linguist/400 | XPages | .xsp-config, .xsp.metadata | text/xml |
| apache-httpd/4000171426298219438 | xpinstall | xpi | application/x-xpinstall |
| apache-httpd/16421235721521251338 | xproc xml | xpl | application/xproc+xml |
| linguist/401 | XProc | .xpl, .xproc | text/xml |
| linguist/402 | XQuery | .xquery, .xq, .xql, .xqm, .xqy | application/xquery |
| apache-httpd/6440939773538770935 | xslt xml | xslt | application/xslt+xml |
| linguist/404 | XSLT | .xslt, .xsl | text/xml |
| apache-httpd/11225691474892377083 | xspf xml | xspf | application/xspf+xml |
| linguist/403 | XS | .xs | text/x-csrc |
| linguist/406 | Xtend | .xtend |  |
| fmt/1497 | XV Thumbnail | p7 |  |
| apache-httpd/1427004451618818651 | xv xml | mxml, xhvml, xvml, xvm | application/xv+xml |
| x-fmt/210 | XYWrite Document | xy |  |
| x-fmt/211 | XYWrite Document | xy3 |  |
| x-fmt/372 | XYWrite Document | xyp |  |
| x-fmt/373 | XYWrite Document | xy4 |  |
| x-fmt/371 | XYWrite for Windows Document | xyw |  |
| apache-httpd/235929493062174822 | xyz | xyz | chemical/x-xyz |
| fmt/1098 | XZ File Format | xz |  |
| apache-httpd/4433712085291521474 | xz | xz | application/x-xz |
| linguist/409 | Yacc | .y, .yacc, .yy |  |
| apache-httpd/10771167679637303936 | yamaha hv dic | hvd | application/vnd.yamaha.hv-dic |
| apache-httpd/3755495095313444404 | yamaha hv script | hvs | application/vnd.yamaha.hv-script |
| apache-httpd/15459861044985438947 | yamaha hv voice | hvp | application/vnd.yamaha.hv-voice |
| apache-httpd/10480217407537137783 | yamaha openscoreformat osfpvg xml | osfpvg | application/vnd.yamaha.openscoreformat.osfpvg+xml |
| apache-httpd/10227406980888024354 | yamaha openscoreformat | osf | application/vnd.yamaha.openscoreformat |
| fmt/1870 | Yamaha PSR Disk Manager File | mng |  |
| apache-httpd/11137728069844962273 | yamaha smaf audio | saf | application/vnd.yamaha.smaf-audio |
| apache-httpd/2616789489216992815 | yamaha smaf phrase | spf | application/vnd.yamaha.smaf-phrase |
| fmt/1662 | Yamaha TX Wave Audio | txw, w01, w02, w03, w04, w05, w06, w07, w08, w09, w10, w11, w12, w13, w14, w15, w16, w17, w18, w19, w20, w21, w22 |  |
| fmt/1661 | Yamaha Wave Audio | s01, u01, f01, w01 |  |
| fmt/818 | YAML | yaml, yml |  |
| linguist/407 | YAML | .yml, .mir, .reek, .rviz, .sublime-syntax, .syntax, .yaml, .yaml-tmlanguage, .yaml.sed, .yml.mysql | text/x-yaml |
| apache-httpd/12313980582379923598 | yang | yang | application/yang |
| linguist/408 | YANG | .yang |  |
| fmt/1663 | YAODL (Yet Another Object Description Language) File | ydl |  |
| linguist/805122868 | YARA | .yar, .yara |  |
| linguist/378760102 | YASnippet | .yasnippet |  |
| apache-httpd/18354551720029817871 | yellowriver custom menu | cmp | application/vnd.yellowriver-custom-menu |
| fmt/1100 | yEnc Encoded File | yenc |  |
| apache-httpd/8558864974377691069 | yin xml | yin | application/yin+xml |
| linguist/237469033 | Yul | .yul |  |
| fmt/1671 | Z Compressed Data | z |  |
| fmt/1627 | Z Print Build File | zbd |  |
| linguist/952972794 | ZAP | .zap, .xzap |  |
| fmt/1673 | ZBrush MatCap | zmt |  |
| linguist/40 | Zeek | .zeek, .bro |  |
| linguist/494938890 | ZenScript | .zs |  |
| linguist/410 | Zephir | .zep |  |
| fmt/1242 | ZFO (Form) File | zfo | application/vnd.software602.filler.form-xml-zip |
| fmt/1243 | ZFO (Message) File | zfo | application/vnd.software602.filler.form-xml-zip |
| fmt/1245 | ZFO (Proof of Delivery) File | zfo | application/vnd.software602.filler.form-xml-zip |
| fmt/1244 | ZFO (Sent Message) File | zfo | application/vnd.software602.filler.form-xml-zip |
| linguist/646424281 | Zig | .zig, .zig.zon |  |
| linguist/973483626 | ZIL | .zil, .mud |  |
| linguist/411 | Zimpl | .zimpl, .zmpl, .zpl |  |
| x-fmt/263 | ZIP Format | zip | application/zip |
| fmt/1143 | ZISRAW (CZI) File Format | czi |  |
| apache-httpd/17719390468068402583 | zmachine | z1, z2, z3, z4, z5, z6, z7, z8 | application/x-zmachine |
| fmt/1193 | ZModeler Z3D | z3d |  |
| fmt/1194 | ZModeler Z3D | z3d |  |
| fmt/1195 | ZModeler Z3D | z3d |  |
| fmt/1213 | Zoner Callisto Metafile | zmf |  |
| x-fmt/269 | ZOO Compressed Archive | zoo |  |
| fmt/1953 | Zoom Project Settings | hprj |  |
| fmt/1954 | Zoom Project Settings | hprj |  |
| fmt/1496 | ZoomBrowser Ex Thumbnail Cache | info |  |
| fmt/898 | Zoomify Image Format | zif |  |
| fmt/756 | Zope Export File | zexp |  |
| fmt/1097 | ZPAQ Archive Format | zpaq |  |
| apache-httpd/15124211333922682931 | zul | zir, zirz | application/vnd.zul |
| fmt/1674 | ZyXEL Voice Format Audio | zvd, zyx, ad2 |  |
| apache-httpd/12435334305085697032 | zzazz deck xml | zaz | application/vnd.zzazz.deck+xml |
<!--FILE_TYPES_END-->

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

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
