use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205449: FileFormat = FileFormat {
    id: 28_205_449,
    source_type: SourceType::Wikidata,
    name: "Design rule for Camera File system THM",
    extensions: &["thm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
