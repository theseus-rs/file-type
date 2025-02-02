use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_97033379: FileFormat = FileFormat {
    id: 97_033_379,
    source_type: SourceType::Wikidata,
    name: "Magick Persistent Registry",
    extensions: &["mpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
