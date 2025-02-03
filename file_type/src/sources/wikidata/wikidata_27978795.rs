use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27978795: FileFormat = FileFormat {
    id: 27_978_795,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Uncompressed",
    extensions: &["spu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
