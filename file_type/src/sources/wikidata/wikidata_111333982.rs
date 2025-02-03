use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333982: FileFormat = FileFormat {
    id: 111_333_982,
    source_type: SourceType::Wikidata,
    name: "Rockwell 4-bit ADPCM data",
    extensions: &["rockwell-4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
