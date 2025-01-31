use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45315902: FileFormat = FileFormat {
    id: 45_315_902,
    puid: "wikidata/45315902",
    name: "Macromedia Freehand file format, version 8",
    extensions: &["fh8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
