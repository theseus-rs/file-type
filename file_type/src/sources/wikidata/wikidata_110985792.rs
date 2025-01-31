use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110985792: FileFormat = FileFormat {
    id: 110_985_792,
    puid: "wikidata/110985792",
    name: "Twin VQ format",
    extensions: &["vqf"],
    media_types: &["audio/x-twinvq"],
    internal_signatures: &[],
    related_formats: &[],
};
