use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58103465: FileFormat = FileFormat {
    id: 58_103_465,
    source_type: SourceType::Wikidata,
    name: "Adobe Font List",
    extensions: &["lst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
