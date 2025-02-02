use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126960131: FileFormat = FileFormat {
    id: 126_960_131,
    source_type: SourceType::Wikidata,
    name: "Standard ML source code file",
    extensions: &["sml"],
    media_types: &["application/x-standardml", "text/x-standardml"],
    internal_signatures: &[],
    related_formats: &[],
};
