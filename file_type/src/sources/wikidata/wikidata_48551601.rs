use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48551601: FileFormat = FileFormat {
    id: 48_551_601,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Windows Macro",
    extensions: &["wpm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
