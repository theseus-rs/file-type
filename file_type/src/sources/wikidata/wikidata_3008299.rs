use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3008299: FileFormat = FileFormat {
    id: 3_008_299,
    source_type: SourceType::Wikidata,
    name: "xorg.conf",
    extensions: &["xorg.conf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
