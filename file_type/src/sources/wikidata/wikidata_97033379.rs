use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97033379: FileFormat = FileFormat {
    id: 97_033_379,
    source_type: SourceType::Wikidata,
    name: "Magick Persistent Registry",
    extensions: &["mpr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
