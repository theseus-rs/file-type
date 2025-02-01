use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104600902: FileFormat = FileFormat {
    id: 104_600_902,
    puid: "wikidata/104600902",
    name: "KDB",
    extensions: &["kdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
