use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130542831: FileFormat = FileFormat {
    id: 130_542_831,
    puid: "wikidata/130542831",
    name: "Pug file format",
    extensions: &["jade", "jade", "pug", "pug"],
    media_types: &["text/x-jade", "text/x-jade", "text/x-pug", "text/x-pug"],
    internal_signatures: &[],
    related_formats: &[],
};
