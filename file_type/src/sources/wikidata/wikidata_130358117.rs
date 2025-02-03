use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130358117: FileFormat = FileFormat {
    id: 130_358_117,
    source_type: SourceType::Wikidata,
    name: "Mosel source code file",
    extensions: &["mos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
