use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333920: FileFormat = FileFormat {
    id: 111_333_920,
    source_type: SourceType::Wikidata,
    name: "Rockwell 2/3/4-bit ADPCM data",
    extensions: &["rockwell"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
