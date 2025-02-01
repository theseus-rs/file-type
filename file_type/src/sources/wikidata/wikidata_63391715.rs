use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63391715: FileFormat = FileFormat {
    id: 63_391_715,
    puid: "wikidata/63391715",
    name: "Microsoft Works Word Processor Macintosh, version 4",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
