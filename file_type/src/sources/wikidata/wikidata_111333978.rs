use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333978: FileFormat = FileFormat {
    id: 111_333_978,
    source_type: SourceType::Wikidata,
    name: "Rockwell 3-bit ADPCM data",
    extensions: &["rockwell-3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
