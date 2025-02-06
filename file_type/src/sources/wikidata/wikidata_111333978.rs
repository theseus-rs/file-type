use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333978: FileFormat = FileFormat {
    id: 111_333_978,
    source_type: SourceType::Wikidata,
    name: "Rockwell 3-bit ADPCM data",
    extensions: &["rockwell-3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
