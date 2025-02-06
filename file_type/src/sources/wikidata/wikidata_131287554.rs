use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131287554: FileFormat = FileFormat {
    id: 131_287_554,
    source_type: SourceType::Wikidata,
    name: "tcsh script file format",
    extensions: &["csh", "tcsh"],
    media_types: &["application/x-csh"],
    signatures: &[],
    related_formats: &[],
};
