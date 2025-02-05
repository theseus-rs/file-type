use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333845: FileFormat = FileFormat {
    id: 111_333_845,
    source_type: SourceType::Wikidata,
    name: "Rockwell ADPCM format (HotFax/Quicklink)",
    extensions: &["rif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
