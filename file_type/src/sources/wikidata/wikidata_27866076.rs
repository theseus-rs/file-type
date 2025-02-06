use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27866076: FileFormat = FileFormat {
    id: 27_866_076,
    source_type: SourceType::Wikidata,
    name: "ABC notation, version 2.0",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc"],
    signatures: &[],
    related_formats: &[],
};
