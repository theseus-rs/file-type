use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333938: FileFormat = FileFormat {
    id: 111_333_938,
    source_type: SourceType::Wikidata,
    name: "Rockwell 2-bit ADPCM data",
    extensions: &["rockwell-2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
