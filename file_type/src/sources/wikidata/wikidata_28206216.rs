use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206216: FileFormat = FileFormat {
    id: 28_206_216,
    source_type: SourceType::Wikidata,
    name: "GrayPaint",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
