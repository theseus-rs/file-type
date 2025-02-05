use crate::format::report_format_detail::ReportFormatDetail;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "PRONOM-Report")]
pub(crate) struct PronomReport {
    #[serde(rename = "report_format_detail")]
    pub(crate) detail: ReportFormatDetail,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;
    use quick_xml::de::from_str;

    #[expect(clippy::too_many_lines)]
    #[test]
    fn test_deserialize() -> Result<()> {
        let xml = indoc! {r#"
            <?xml version="1.0" encoding="utf-8"?>
            <PRONOM-Report xmlns="http://pronom.nationalarchives.gov.uk">
              <report_format_detail>
                <FileFormat>
                  <FormatID>766</FormatID>
                  <FormatName>Drawing Interchange File Format (ASCII)</FormatName>
                  <FormatVersion>Generic</FormatVersion>
                  <FormatAliases>ASCII DXF (Generic)</FormatAliases>
                  <FormatFamilies>DXF</FormatFamilies>
                  <FormatTypes>Image (Vector)</FormatTypes>
                  <FormatDisclosure>None</FormatDisclosure>
                  <FormatDescription>ASCII DXF format is the ASCII encoded variant of the Drawing Interchange File Format, an exchange format for vector graphics developed by AutoDesk. They are probably the widely-used format for exchanging vector data, and have become a de-facto industry standard. The format is owned by AutoDesk and typically changes with each release of their AutoCAD family of products. DXF files contain a tagged representation of the vector data. The structure comprises a Header section, followed by sections containing data on Classes, Tables, Blocks, Entities and, optionally, thumbnail images. Each section contains a series of data elements. each preceded by a group code tag, which indicates the type of data element. It is not mandatory for ASCII DXF files to include a version identifier - some DXF files may therefore be identifiable only as generic DXF.</FormatDescription>
                  <BinaryFileFormat>Text</BinaryFileFormat>
                  <ByteOrders>
                  </ByteOrders>
                  <ReleaseDate>
                  </ReleaseDate>
                  <WithdrawnDate>
                  </WithdrawnDate>
                  <ProvenanceSourceID>1</ProvenanceSourceID>
                  <ProvenanceName>Digital Preservation Department / The National Archives</ProvenanceName>
                  <ProvenanceSourceDate>22 Apr 2005</ProvenanceSourceDate>
                  <ProvenanceDescription>
                  </ProvenanceDescription>
                  <LastUpdatedDate>28 Sep 2020</LastUpdatedDate>
                  <FormatNote>
                  </FormatNote>
                  <FormatRisk>
                  </FormatRisk>
                  <TechnicalEnvironment>
                  </TechnicalEnvironment>
                  <FileFormatIdentifier>
                    <Identifier>fmt/63</Identifier>
                    <IdentifierType>PUID</IdentifierType>
                  </FileFormatIdentifier>
                  <FileFormatIdentifier>
                    <Identifier>image/vnd.dxf</Identifier>
                    <IdentifierType>MIME</IdentifierType>
                  </FileFormatIdentifier>
                  <Developers>
                    <DeveloperID>18</DeveloperID>
                    <DeveloperName>
                    </DeveloperName>
                    <OrganisationName>Autodesk</OrganisationName>
                    <DeveloperCompoundName>Autodesk</DeveloperCompoundName>
                  </Developers>
                  <Support>
                    <SupportID>18</SupportID>
                    <SupportName>
                    </SupportName>
                    <OrganisationName>Autodesk</OrganisationName>
                    <SupportCompoundName>Autodesk</SupportCompoundName>
                  </Support>
                  <ExternalSignature>
                    <ExternalSignatureID>757</ExternalSignatureID>
                    <Signature>dxf</Signature>
                    <SignatureType>File extension</SignatureType>
                  </ExternalSignature>
                  <InternalSignature>
                    <SignatureID>115</SignatureID>
                    <SignatureName>Drawing Interchange File Format (ASCII) (generic)</SignatureName>
                    <SignatureNote>Section group code, EOF marker</SignatureNote>
                    <ByteSequence>
                      <ByteSequenceID>17</ByteSequenceID>
                      <PositionType>Variable</PositionType>
                      <Offset>
                      </Offset>
                      <MaxOffset>
                      </MaxOffset>
                      <IndirectOffsetLocation>
                      </IndirectOffsetLocation>
                      <IndirectOffsetLength>
                      </IndirectOffsetLength>
                      <Endianness>
                      </Endianness>
                      <ByteSequenceValue>30(0D0A|0A|0D)53454354494F4E(0D0A|0A|0D){0-5}32(0D0A|0A|0D)*30(0D0A|0A|0D)454E44534543</ByteSequenceValue>
                    </ByteSequence>
                    <ByteSequence>
                      <ByteSequenceID>18</ByteSequenceID>
                      <PositionType>Absolute from EOF</PositionType>
                      <Offset>0</Offset>
                      <MaxOffset>5</MaxOffset>
                      <IndirectOffsetLocation>
                      </IndirectOffsetLocation>
                      <IndirectOffsetLength>
                      </IndirectOffsetLength>
                      <Endianness>
                      </Endianness>
                      <ByteSequenceValue>30(0D0A|0A|0D)454F46</ByteSequenceValue>
                    </ByteSequence>
                  </InternalSignature>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>710</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>1.0</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>711</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>1.2</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>712</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>1.3</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>713</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>1.4</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>714</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2.0</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>715</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2.1</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>716</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2.2</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>717</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2.5</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>718</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2.6</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>719</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>R9</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>720</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>R10</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>721</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>R11/12</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>722</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>R13</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>723</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>R14</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>724</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2000-2002</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>725</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2004/2005/2006</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>1220</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2007/2008/2009</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>1221</RelatedFormatID>
                    <RelatedFormatName>AutoCAD Drawing</RelatedFormatName>
                    <RelatedFormatVersion>2010/2011/2012</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>1222</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2010/2011/2012</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>1319</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange File Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2013/2014/2015/2016/2017</RelatedFormatVersion>
                  </RelatedFormat>
                  <RelatedFormat>
                    <RelationshipType>Has lower priority than</RelationshipType>
                    <RelatedFormatID>2207</RelatedFormatID>
                    <RelatedFormatName>Drawing Interchange Format (ASCII)</RelatedFormatName>
                    <RelatedFormatVersion>2018/2019/2020</RelatedFormatVersion>
                  </RelatedFormat>
                </FileFormat>
                <SearchCriteria>Criteria</SearchCriteria>
              </report_format_detail>
            </PRONOM-Report>
        "#};
        let report: PronomReport = from_str(xml)?;
        let detail = report.detail;
        let file_format = detail.file_format;
        assert_eq!(file_format.id, 766);
        Ok(())
    }
}
