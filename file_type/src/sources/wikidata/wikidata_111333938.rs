use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333938: FileFormat = FileFormat {
    id: 111_333_938,
    source_type: SourceType::Wikidata,
    name: "Rockwell 2-bit ADPCM data",
    extensions: &["rockwell-2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
