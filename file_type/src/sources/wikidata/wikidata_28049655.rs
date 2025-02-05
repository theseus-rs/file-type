use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049655: FileFormat = FileFormat {
    id: 28_049_655,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Device-Independent Binary Plotter File",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
