use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71858982: FileFormat = FileFormat {
    id: 71_858_982,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 10",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
