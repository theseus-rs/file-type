use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771320: FileFormat = FileFormat {
    id: 28_771_320,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office File List",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
