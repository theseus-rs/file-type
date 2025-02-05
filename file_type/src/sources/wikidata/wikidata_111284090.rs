use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284090: FileFormat = FileFormat {
    id: 111_284_090,
    source_type: SourceType::Wikidata,
    name: "G.723 3-bit (24 kbps) ADPCM format data",
    extensions: &["g723-3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
