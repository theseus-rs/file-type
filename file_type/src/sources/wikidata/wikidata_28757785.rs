use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757785: FileFormat = FileFormat {
    id: 28_757_785,
    source_type: SourceType::Wikidata,
    name: "GMLJP2",
    extensions: &["jpf", "jpx"],
    media_types: &["image/jpx"],
    signatures: &[],
    related_formats: &[],
};
