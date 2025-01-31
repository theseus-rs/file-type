use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652258: FileFormat = FileFormat {
    id: 112_652_258,
    puid: "wikidata/112652258",
    name: "3ds Max Characters",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
