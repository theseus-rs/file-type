use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119013900: FileFormat = FileFormat {
    id: 119_013_900,
    puid: "wikidata/119013900",
    name: "Binspector grammar",
    extensions: &["bfft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
