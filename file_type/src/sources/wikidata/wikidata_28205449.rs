use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205449: FileFormat = FileFormat {
    id: 28_205_449,
    puid: "wikidata/28205449",
    name: "Design rule for Camera File system THM",
    extensions: &["thm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
