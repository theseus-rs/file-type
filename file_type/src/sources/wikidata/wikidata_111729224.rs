use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111729224: FileFormat = FileFormat {
    id: 111_729_224,
    source_type: SourceType::Wikidata,
    name: "Document Manager file",
    extensions: &["ddm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
