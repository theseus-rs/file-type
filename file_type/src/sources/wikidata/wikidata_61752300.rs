use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61752300: FileFormat = FileFormat {
    id: 61_752_300,
    source_type: SourceType::Wikidata,
    name: "Microsoft FrontPage file format",
    extensions: &["lck"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
