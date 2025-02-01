use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652534: FileFormat = FileFormat {
    id: 112_652_534,
    puid: "wikidata/112652534",
    name: "Astound 1.5 Library file format",
    extensions: &["asl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
