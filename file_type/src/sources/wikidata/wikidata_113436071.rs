use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113436071: FileFormat = FileFormat {
    id: 113_436_071,
    puid: "wikidata/113436071",
    name: "INTREPID Standard Information File",
    extensions: &["isi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
