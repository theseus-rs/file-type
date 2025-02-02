use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333845: FileFormat = FileFormat {
    id: 111_333_845,
    source_type: SourceType::Wikidata,
    name: "Rockwell ADPCM format (HotFax/Quicklink)",
    extensions: &["rif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
