use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58103465: FileFormat = FileFormat {
    id: 58_103_465,
    source_type: SourceType::Wikidata,
    name: "Adobe Font List",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
