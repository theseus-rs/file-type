use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131860302: FileFormat = FileFormat {
    id: 131_860_302,
    source_type: SourceType::Wikidata,
    name: "MNI transformation file format",
    extensions: &["xfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
