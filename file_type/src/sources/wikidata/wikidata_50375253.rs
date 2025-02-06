use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50375253: FileFormat = FileFormat {
    id: 50_375_253,
    source_type: SourceType::Wikidata,
    name: "Extensible Metadata Platform Format",
    extensions: &["xmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
