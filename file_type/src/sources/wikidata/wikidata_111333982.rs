use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333982: FileFormat = FileFormat {
    id: 111_333_982,
    source_type: SourceType::Wikidata,
    name: "Rockwell 4-bit ADPCM data",
    extensions: &["rockwell-4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
