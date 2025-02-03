use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48551601: FileFormat = FileFormat {
    id: 48_551_601,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Windows Macro",
    extensions: &["wpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
