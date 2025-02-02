use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205449: FileFormat = FileFormat {
    id: 28_205_449,
    source_type: SourceType::Wikidata,
    name: "Design rule for Camera File system THM",
    extensions: &["thm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
