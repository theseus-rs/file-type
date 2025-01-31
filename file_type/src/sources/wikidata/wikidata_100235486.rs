use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100235486: FileFormat = FileFormat {
    id: 100_235_486,
    puid: "wikidata/100235486",
    name: "AutoCAD Drawing, version 2018/2019/2020/2021/2022/2023",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
