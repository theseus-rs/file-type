use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27978795: FileFormat = FileFormat {
    id: 27_978_795,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Uncompressed",
    extensions: &["spu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
